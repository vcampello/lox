# Lox

A work-in-progress Rust implementation of the Lox programming language from [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Goals

To implement the following:

- Interpreter
- Formatter
- LSP
- Debugger
- MCP server into each of the above

## Status

🚧 This is an educational project and a work in progress.

## Usage

Run a Lox script:

```bash
cargo run -- script.lox
```

Start the REPL:

```bash
cargo run
```

## Project Structure

- `src/scanner.rs` - Lexical analysis
- `src/token.rs` - Token definitions
- `src/ast/` - Abstract syntax tree
- `src/runtime.rs` - Runtime interpreter
- `samples/` - Example Lox programs

## License

MIT License - see [LICENSE](LICENSE) for details.

This project is based on the Lox language specification from [Crafting Interpreters](https://craftinginterpreters.com/).
