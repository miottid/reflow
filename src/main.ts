import Anthropic from '@anthropic-ai/sdk'

const apiKey = process.env.ANTHROPIC_API_KEY

if (!apiKey) {
    console.error('Error: ANTHROPIC_API_KEY environment variable is not set')
    console.error('Please set it with: export ANTHROPIC_API_KEY=your_api_key')
    process.exit(1)
}

const text = process.argv[2]

if (!text) {
    console.error('Error: Please provide text to reformat')
    console.error('Usage: write-better "your text here"')
    process.exit(1)
}

const client = new Anthropic({ apiKey })

const prompt =
    'Please translate from French to English and reformat the following text to be:\n' +
    '- Succinct and clear\n' +
    '- Professional and corporate-appropriate\n' +
    '- With a subtle touch of irony when appropriate\n\n' +
    'Original text: ' +
    text +
    '\n\n' +
    'Return only the reformatted text, without any explanation or preamble.'

async function main() {
    try {
        const message = await client.messages.create({
            model: 'claude-sonnet-4-5-20250929',
            max_tokens: 1024,
            messages: [
                {
                    role: 'user',
                    content: prompt,
                },
            ],
        })

        const response = message.content[0]
        if (response.type === 'text') {
            console.log(response.text)
        }
    } catch (error) {
        console.error('Error calling Claude API:', error)
        process.exit(1)
    }
}

main()
