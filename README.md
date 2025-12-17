# Reflow

A CLI tool that uses Claude AI to reformat text, making it more succinct, clear, and professional. It also translates French text to English automatically.

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
