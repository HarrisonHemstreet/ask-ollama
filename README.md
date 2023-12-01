# Ask-Ollama

Ask-Ollama is a command-line tool that allows users to interact with the [Ollama](https://ollama.ai/) LLM models directly from the terminal. This tool provides a simple and intuitive way to ask questions and receive responses from Ollama models.

## Features

- **Interactive CLI**: Easily ask questions and get responses.
- **Model Selection**: Choose different models for varied responses.

## Installation

To install Ask-Ollama, you need to have Rust and Cargo installed on your system. If you haven't already installed Rust, you can do so by following the instructions [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install Ask-Ollama using Cargo:

```sh
cargo install ask-ollama
```

## Usage

After installation, you can start using Ask-Ollama by running:

```sh
ask [OPTIONS] [PROMPT]
```

### Options

- `--model=[MODEL]`: Specify the model to use (default is 'mistral').
- `--version`: Display the version of the installed Ask-Ollama tool.
- `[PROMPT]`: The question or prompt to send to Ollama. Quotation marks are not required.

### Examples

Asking a question using the default model:

```sh
ask "What is the capital of France?"
```
or
```sh
ask What is the capital of France?
```

Specifying a different model:

```sh
ask --model=gale "Explain the theory of relativity"
```
Find all available models from Ollama [here](https://ollama.ai/library).

Checking the version:

```sh
ask --version
```

Seeing the help info:
```sh
ask --help
```

## Contributing

Contributions to Ask-Ollama are welcome! If you have suggestions for improvements or encounter any issues, please feel free to open an issue or submit a pull request on our [GitHub repository](https://github.com/HarrisonHemstreet/ask-ollama).

## License

Ask-Ollama is licensed under the MIT License.
