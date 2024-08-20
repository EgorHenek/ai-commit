# AI Commit

AI Commit is a command-line tool that generates commit messages using AI. It supports multiple AI providers and models to help you create meaningful and consistent commit messages for your Git repositories.

## Features

- Generate commit messages based on git diffs
- Support for multiple AI providers (OpenAI and OpenRouter)
- Customizable AI models
- List available models for each provider

## Installation

There are several ways to install AI Commit:

### Using Cargo (recommended)

If you have Rust and Cargo installed, you can install AI Commit directly from crates.io:

```
cargo install ai-commit
```

This will download, compile, and install the latest version of AI Commit.

### Pre-built Binaries

You can also download pre-built binaries for your system from the [latest release](https://github.com/EgorHenek/ai-commit/releases/latest) on GitHub.

1. Download the appropriate archive for your system:
   - For macOS (Intel): `ai-commit-x86_64-apple-darwin.tar.gz`
   - For macOS (Apple Silicon): `ai-commit-aarch64-apple-darwin.tar.gz`
   - For Linux (x86_64): `ai-commit-x86_64-unknown-linux-gnu.tar.gz`
   - For Linux (ARM64): `ai-commit-aarch64-unknown-linux-gnu.tar.gz`
   - For Windows: `ai-commit-x86_64-pc-windows-msvc.zip`

2. Extract the archive:
   - For `.tar.gz` files: `tar xzf ai-commit-<version>.tar.gz`
   - For `.zip` files: Unzip using your preferred tool

3. Move the `ai-commit` binary to a directory in your PATH, for example:
   ```
   sudo mv ai-commit /usr/local/bin/
   ```

### Building from Source

If you prefer to build from source, make sure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```
git clone https://github.com/EgorHenek/ai-commit.git
cd ai-commit
cargo build --release
```

The compiled binary will be available in `target/release/ai-commit`.

## Usage

To use AI Commit, you need to set up your API keys for the AI providers you want to use. You can do this by setting environment variables:

- For OpenAI: `OPENAI_API_KEY`
- For OpenRouter: `OPENROUTER_API_KEY`

### Basic usage

```
git diff | ai-commit
```

This will generate a commit message based on the current git diff using the default provider (OpenAI) and model.

### Specifying a provider and model

```
git diff | ai-commit --provider openrouter --model gpt-3.5-turbo
```

### Listing available models

```
ai-commit --provider openai --list-models
```

### Full command-line options

```
ai-commit --help
```

## Configuration

You can set default values for the AI model and provider using environment variables:

- `AI_MODEL`: Sets the default AI model
- `OPENAI_API_KEY`: Sets the API key for OpenAI
- `OPENROUTER_API_KEY`: Sets the API key for OpenRouter

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
