# Reflow

A CLI tool that uses Claude AI to improve text, making it clear, concise, and professional while preserving its original meaning.

## Requirements

- [Bun](https://bun.sh/) runtime
- Anthropic API key

## Installation

```bash
bun install
```

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
./reflow.ts
```

Type your text, then press `Ctrl+D` to submit. Press `Ctrl+C` to exit.

### Piped Input

You can also pipe text directly:

```bash
echo "Your text here" | ./reflow.ts
```

Or from a file:

```bash
cat myfile.txt | ./reflow.ts
```

## Building

Compile to a standalone executable:

```bash
bun run build
```

This creates `dist/reflow` which can be run directly:

```bash
./dist/reflow
```

## License

ISC
