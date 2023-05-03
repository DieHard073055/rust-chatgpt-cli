# Rust ChatGPT CLI

## In use

[<img src="https://i.ytimg.com/vi/d-WKDH4-6sI/maxresdefault.jpg">](https://www.youtube.com/watch?v=d-WKDH4-6sI "Script Output")

# rust-chatgpt-cli

`rust-chatgpt-cli` is a command-line interface (CLI) application that allows you to interact with OpenAI's ChatGPT API. You can start new conversations, list and delete existing ones, and process chatbot prompts.

## Usage Instructions

### Prerequisites

- Rust installed on your system
- OpenAI API Key

## Getting Started

### Build and Run

1. Clone this repository:
```
git clone https://github.com/yourusername/rust-chatgpt-cli.git
```

2. Set your OpenAI API key as an environment variable:
```shell
export OPENAI_KEY=your_openai_key
```

3. Navigate to the project directory:
```
cd rust-chatgpt-cli
```

4. Build the project:
```
cargo build --release
```

5. Run the application:
```
./target/release/rust-chatgpt-cli [FLAGS] [OPTIONS]
```

### Flags and Options

- `-n, --new-conversation`: Start a new conversation
- `-l, --list`: List all conversations
- `-d, --del INDEX`: Delete a conversation by its index
- `prompt`: Enter your prompt (can be a single or multiple words)

### Examples

All the features with (🚧) are under still under construction.

1. Start a new conversation (🚧):
```
./target/release/rust-chatgpt-cli -n What is the meaning of life?
```

2. List all conversations (🚧):
```
./target/release/rust-chatgpt-cli -l
```

3. Delete a conversation by index (🚧):
```
./target/release/rust-chatgpt-cli -d 2
```

4. Provide a prompt:
```
./target/release/rust-chatgpt-cli Tell me a joke.
```

5. Use stdin for the prompt:
```
echo "What is your favorite color?" | ./target/release/rust-chatgpt-cli
```

6. Use a file to build your prompt, then pipe it to chatgpt.
```shell
$ cat > prompt
You are a computer science lecturer at a university.
You excel at explaining programming concepts to student.

Could you please explain what a hashtable is?
^C
$ cat prompt | ./target/release/rust-chatgpt-cli
```

Note: (🚧) You can combine the `-n` flag with a prompt to start a new conversation with the given prompt.

## Contributing

Please feel free to open an issue or submit a pull request with your contributions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
