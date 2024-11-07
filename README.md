# Brainalyzer

**Brainalyzer** is a powerful, minimalist parser and interpreter for the Brainfuck programming language. Designed for performance and ease of understanding, Brainalyzer offers users and developers a streamlined interface for executing, debugging, and analyzing Brainfuck code.

## Features

- **Efficient Parsing:** Fast and robust parsing of Brainfuck code with optimized memory handling.
- **Error Reporting:** Helpful error messages that pinpoint syntax and logic issues in Brainfuck programs.
- **Step-by-Step Execution:** A feature to execute code step-by-step, ideal for debugging and learning.
- **Customizable Memory Tape:** Configure memory cell size and length to simulate different environments.
- **Flexible Output:** Display program output in the console, or redirect it to files or other processes.

## Installation

To install Brainalyzer, clone the repository and compile the source code:

```bash
git clone https://github.com/atontini/brainalyzer.git
cd brainalyzer
cargo build --release
```

## Usage

Brainalyzer can be used both interactively and with pre-written Brainfuck files.

### Running Brainfuck Code

To run a Brainfuck file:

```bash
brainalyzer run path/to/yourfile.bf
```

In interactive mode, enter your Brainfuck code line-by-line, and Brainalyzer will interpret it in real-time.

## Examples

Execute a Brainfuck program stored in `examples/hello.bf`:

```bash
brainalyzer run examples/hello.bf
```

## Contributing

Contributions are welcome! Please submit a pull request with a clear description of your changes, and ensure that all tests pass before submitting.