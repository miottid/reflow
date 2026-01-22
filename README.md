# Reflow

A CLI tool that uses Claude AI to improve text, making it clear, concise, and professional while preserving its original meaning.

## Requirements

- Rust
- Anthropic API key

## Installation

Install dependencies and build:

```bash
cargo build --release
```

The compiled binary will be at `target/release/reflow`.

## Configuration

Set your Anthropic API key as an environment variable:

```bash
export ANTHROPIC_API_KEY=your_api_key
```

### Custom Prompt

By default, Reflow uses a built-in prompt for improving text. You can customize this by creating a `~/reflow.txt` file with your own prompt:

```bash
echo "Your custom prompt here" > ~/reflow.txt
```

If the file exists, its contents will be used as the prompt prefix. Delete the file to revert to the default behavior.

## Usage

### Interactive Mode

Run the tool directly to enter interactive mode:

```bash
./target/release/reflow
# or during development:
cargo run
```

Type your text, then press `Ctrl+D` to submit. Press `Ctrl+C` to exit.

### Piped Input

You can also pipe text directly:

```bash
echo "Your text here" | ./target/release/reflow
# or:
cargo run < myfile.txt
```

## Building

Compile to an optimized standalone executable (~1.3 MB):

```bash
cargo build --release
```

This creates `target/release/reflow` which can be run directly or installed:

```bash
# Run directly
./target/release/reflow

# Or install to system
cargo install --path .
```

## License

ISC
