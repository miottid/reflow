#!/usr/bin/env bun
import Anthropic from '@anthropic-ai/sdk'
import { existsSync, readFileSync } from 'node:fs'
import { homedir } from 'node:os'
import { join } from 'node:path'

const apiKey = process.env.ANTHROPIC_API_KEY

if (!apiKey) {
    console.error('Error: ANTHROPIC_API_KEY environment variable is not set')
    console.error('Please set it with: export ANTHROPIC_API_KEY=your_api_key')
    process.exit(1)
}

const client = new Anthropic({ apiKey })

const defaultPromptPrefix =
    'Improve the following text.\n\n' +
    'Rules:\n' +
    '- Preserve the original meaning and essence\n' +
    '- Fix any grammar, spelling, or punctuation errors\n' +
    '- Keep the tone professional and respectful\n' +
    '- Be clear and concise without changing the substance\n\n' +
    'Return only the improved text, without any explanation.\n\n' +
    'Original text: '

function loadPromptPrefix(): string {
    const configPath = join(homedir(), 'reflow.txt')
    if (existsSync(configPath)) {
        return readFileSync(configPath, 'utf8').trim()
    }
    return defaultPromptPrefix
}

const promptPrefix = loadPromptPrefix()

function buildPrompt(text: string): string {
    return (
        promptPrefix +
        text.trim() +
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
