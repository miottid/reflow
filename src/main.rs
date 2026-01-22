use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use is_terminal::IsTerminal;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

// Constants
const API_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-5-20250929";
const MAX_TOKENS: u32 = 1024;
const API_VERSION: &str = "2023-06-01";

const DEFAULT_PROMPT_PREFIX: &str = "Improve the following text.\n\n\
    Rules:\n\
    - Preserve the original meaning and essence\n\
    - Fix any grammar, spelling, or punctuation errors\n\
    - Keep the tone professional and respectful\n\
    - Be clear and concise without changing the substance\n\n\
    Return only the improved text, without any explanation.\n\n\
    Original text: ";

// Anthropic API Types
#[derive(Serialize)]
struct MessageRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}

fn load_prompt_prefix() -> Result<String> {
    let home = dirs::home_dir().context("Failed to find home directory")?;
    let config_path = home.join("reflow.txt");

    if config_path.exists() {
        std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))
            .map(|s| s.trim().to_string())
    } else {
        Ok(DEFAULT_PROMPT_PREFIX.to_string())
    }
}

fn build_prompt(text: &str, prefix: &str) -> String {
    format!(
        "{}{}\n\nReturn only the reformatted text, without any explanation or preamble.",
        prefix,
        text.trim()
    )
}

fn call_claude(text: &str, api_key: &str, prompt_prefix: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();

    let request = MessageRequest {
        model: MODEL.to_string(),
        max_tokens: MAX_TOKENS,
        messages: vec![Message {
            role: "user".to_string(),
            content: build_prompt(text, prompt_prefix),
        }],
    };

    let response = client
        .post(API_ENDPOINT)
        .header("Content-Type", "application/json")
        .header("anthropic-version", API_VERSION)
        .header("x-api-key", api_key)
        .json(&request)
        .send()
        .context("Failed to send request to Claude API")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("API request failed with status {}: {}", status, error_text);
    }

    let message_response: MessageResponse =
        response.json().context("Failed to parse API response")?;

    let text_response = message_response
        .content
        .into_iter()
        .find(|block| block.block_type == "text")
        .and_then(|block| block.text)
        .context("No text content in API response")?;

    Ok(text_response)
}

fn handle_piped_input(api_key: &str, prompt_prefix: &str) -> Result<()> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    let text = buffer.trim();
    if text.is_empty() {
        eprintln!("No text provided on stdin.");
        std::process::exit(1);
    }

    match call_claude(text, api_key, prompt_prefix) {
        Ok(response) => {
            println!("\n{}\n", response);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error calling Claude API: {}", e);
            std::process::exit(1);
        }
    }
}

fn start_interactive_loop(api_key: &str, prompt_prefix: &str) -> Result<()> {
    let mut buffer = String::new();

    let print_prompt = || {
        println!("Enter text (Ctrl+D to submit, Ctrl+C to exit):");
        io::stdout().flush().unwrap();
    };

    let submit = |buffer: &mut String| -> Result<()> {
        let text = buffer.trim().to_string();
        *buffer = String::new();

        if text.is_empty() {
            print!("\nNo text provided. Keep typing then hit Ctrl+D.\n\n");
            print_prompt();
            return Ok(());
        }

        print!("\n^D\n");
        io::stdout().flush()?;

        match call_claude(&text, api_key, prompt_prefix) {
            Ok(response) => {
                println!("\n{}\n", response);
            }
            Err(e) => {
                eprintln!("Error calling Claude API: {}", e);
            }
        }

        print_prompt();
        Ok(())
    };

    enable_raw_mode().context("Failed to enable raw mode")?;

    // Ensure we disable raw mode on exit
    let result = (|| -> Result<()> {
        print_prompt();

        loop {
            let event = event::read().context("Failed to read event")?;

            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event
            {
                match code {
                    // Ctrl+C exits
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        print!("\nExiting.\n");
                        io::stdout().flush()?;
                        break;
                    }
                    // Ctrl+D submits
                    KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
                        submit(&mut buffer)?;
                    }
                    // Backspace
                    KeyCode::Backspace => {
                        if !buffer.is_empty() {
                            buffer.pop();
                            print!("\x08 \x08");
                            io::stdout().flush()?;
                        }
                    }
                    // Enter key
                    KeyCode::Enter => {
                        buffer.push('\n');
                        println!();
                        io::stdout().flush()?;
                    }
                    // Regular character
                    KeyCode::Char(c) => {
                        buffer.push(c);
                        print!("{}", c);
                        io::stdout().flush()?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode().context("Failed to disable raw mode")?;
    result
}

fn main() -> Result<()> {
    // Check for API key
    let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: ANTHROPIC_API_KEY environment variable is not set");
        eprintln!("Please set it with: export ANTHROPIC_API_KEY=your_api_key");
        std::process::exit(1);
    });

    // Load prompt prefix
    let prompt_prefix = load_prompt_prefix().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load custom prompt: {}", e);
        eprintln!("Using default prompt.");
        DEFAULT_PROMPT_PREFIX.to_string()
    });

    // Check if stdin is a TTY
    if io::stdin().is_terminal() {
        start_interactive_loop(&api_key, &prompt_prefix)?;
    } else {
        handle_piped_input(&api_key, &prompt_prefix)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_prompt_prefix_not_empty() {
        assert!(!DEFAULT_PROMPT_PREFIX.is_empty());
        assert!(DEFAULT_PROMPT_PREFIX.contains("Improve the following text"));
    }

    #[test]
    fn test_build_prompt_formats_correctly() {
        let prefix = "Test prefix: ";
        let text = "  test text  ";
        let result = build_prompt(text, prefix);

        assert!(result.starts_with("Test prefix: "));
        assert!(result.contains("test text"));
        assert!(result
            .ends_with("Return only the reformatted text, without any explanation or preamble."));
        // Verify text is trimmed
        assert!(!result.contains("  test text  "));
    }

    #[test]
    fn test_build_prompt_handles_empty_text() {
        let prefix = "Prefix: ";
        let text = "   ";
        let result = build_prompt(text, prefix);

        // Should still build a valid prompt even with empty/whitespace text
        assert!(result.contains("Prefix: "));
        assert!(result.contains("Return only the reformatted text"));
    }

    #[test]
    fn test_build_prompt_preserves_newlines() {
        let prefix = "Prefix: ";
        let text = "line1\nline2\nline3";
        let result = build_prompt(text, prefix);

        assert!(result.contains("line1\nline2\nline3"));
    }

    #[test]
    fn test_message_request_serialization() {
        let request = MessageRequest {
            model: MODEL.to_string(),
            max_tokens: MAX_TOKENS,
            messages: vec![Message {
                role: "user".to_string(),
                content: "test".to_string(),
            }],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains(MODEL));
        assert!(serialized.contains("\"max_tokens\":1024"));
        assert!(serialized.contains("\"role\":\"user\""));
        assert!(serialized.contains("\"content\":\"test\""));
    }

    #[test]
    fn test_message_response_deserialization() {
        let json = r#"{
            "content": [
                {
                    "type": "text",
                    "text": "improved text"
                }
            ]
        }"#;

        let response: MessageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.content.len(), 1);
        assert_eq!(response.content[0].block_type, "text");
        assert_eq!(response.content[0].text, Some("improved text".to_string()));
    }

    #[test]
    fn test_constants() {
        assert_eq!(MODEL, "claude-sonnet-4-5-20250929");
        assert_eq!(MAX_TOKENS, 1024);
        assert_eq!(API_VERSION, "2023-06-01");
        assert_eq!(API_ENDPOINT, "https://api.anthropic.com/v1/messages");
    }
}
