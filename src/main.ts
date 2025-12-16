import Anthropic from '@anthropic-ai/sdk'

const apiKey = process.env.ANTHROPIC_API_KEY

if (!apiKey) {
    console.error('Error: ANTHROPIC_API_KEY environment variable is not set')
    console.error('Please set it with: export ANTHROPIC_API_KEY=your_api_key')
    process.exit(1)
}

const client = new Anthropic({ apiKey })

const promptPrefix =
    'Please translate from French to English if it is not already in English and reformat the following text to be:\n' +
    '- Succinct and clear\n' +
    '- Professional and corporate-appropriate\n\n' +
    'Original text: '

function buildPrompt(text: string): string {
    return (
        promptPrefix +
        text +
        '\n\n' +
        'Return only the reformatted text, without any explanation or preamble.'
    )
}

async function callClaude(text: string) {
    try {
        const message = await client.messages.create({
            model: 'claude-sonnet-4-5-20250929',
            max_tokens: 1024,
            messages: [
                {
                    role: 'user',
                    content: buildPrompt(text),
                },
            ],
        })

        const response = message.content[0]
        if (response.type === 'text') {
            console.log(`\n${response.text}\n`)
        }
    } catch (error) {
        console.error('Error calling Claude API:', error)
    }
}

function handlePipedInput() {
    let data = ''
    process.stdin.setEncoding('utf8')
    process.stdin.on('data', (chunk) => {
        data += chunk
    })
    process.stdin.on('end', async () => {
        const text = data.trim()
        if (!text) {
            console.error('No text provided on stdin.')
            process.exit(1)
        }
        await callClaude(text)
    })
}

function startInteractiveLoop() {
    let buffer = ''
    let isProcessing = false

    const printPrompt = () => {
        process.stdout.write('Enter text (Ctrl+D to submit, Ctrl+C to exit):\n')
    }

    const submit = async () => {
        if (isProcessing) {
            return
        }

        const text = buffer.trim()
        buffer = ''

        if (!text) {
            process.stdout.write('\nNo text provided. Keep typing then hit Ctrl+D.\n\n')
            printPrompt()
            return
        }

        isProcessing = true
        process.stdout.write('\n^D\n')
        await callClaude(text)
        isProcessing = false
        printPrompt()
    }

    process.stdin.setEncoding('utf8')
    if (process.stdin.isTTY) {
        process.stdin.setRawMode(true)
    }

    process.stdin.on('data', (chunk: string) => {
        for (const char of chunk) {
            // Ctrl+C exits
            if (char === '\u0003') {
                process.stdout.write('\nExiting.\n')
                process.exit(0)
            }

            // Ctrl+D submits current buffer
            if (char === '\u0004') {
                void submit()
                continue
            }

            // Backspace handling
            if (char === '\u0008' || char === '\u007f') {
                if (buffer.length > 0) {
                    buffer = buffer.slice(0, -1)
                    process.stdout.write('\b \b')
                }
                continue
            }

            // Enter key
            if (char === '\r') {
                buffer += '\n'
                process.stdout.write('\n')
                continue
            }

            buffer += char
            process.stdout.write(char)
        }
    })

    printPrompt()
    process.stdin.resume()
}

if (process.stdin.isTTY) {
    startInteractiveLoop()
} else {
    handlePipedInput()
}
