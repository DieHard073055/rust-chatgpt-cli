# Rust ChatGPT CLI

Rust ChatGPT CLI is a command-line interface for interacting with OpenAI's GPT-based chatbot in the terminal. It enables users to start new conversations, manage ongoing conversations, and send text from a file to the chatbot using simple commands.

## Features

- Start a new conversation with the ChatGPT API by entering a message: `chatgpt new "your message"`
- Send the contents of a file to the ChatGPT API: `cat file_with_contents | chatgpt new`
- List all ongoing conversations: `chatgpt list`
- Continue ongoing conversations: `chatgpt cont <index> "your new message"`
- Delete a conversation based on its index: `chatgpt del <index>`

## Prerequisites

- Rust programming language
- OpenAI API Key

## Getting Started

1. Clone the repository and navigate to the project directory.
2. Set your OpenAI API key as an environment variable:
```shell
export OPENAI_KEY=your_openai_key
```

## Usage

- Start a new conversation:
```shell
chatgpt new "your message"
```

- Send the contents of a file to the ChatGPT API:
```shell
cat file_with_contents | chatgpt new
```

- List all ongoing conversations:

```shell
chatgpt list
```

- Continue old conversation by using the index to refer to it.
```shell
chatgpt cont <index> "your message"
```

- Delete a conversation based on its index:
```shell
chatgpt del <index>

```

## Contributing

Please feel free to open an issue or submit a pull request with your contributions.

## License

This project is licensed under the MIT License.
