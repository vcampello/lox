# Lox

A work-in-progress Rust implementation of the Lox programming language from [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Language Features

Currently implemented:
- Variables and assignment
- Control flow: `if`/`else`, `while` loops
- Blocks and lexical scoping
- Expressions: binary operators, unary operators, logical operators (`and`, `or`)
- Literals: numbers, strings, booleans, `nil`
- Comments
- `print` statement

Not yet implemented:
- Functions
- Classes
- `for` loops
- `break`/`continue`

## Goals

To implement the following:

- Interpreter - in progress 
- Formatter
- LSP
- Debugger
- MCP server into each of the above

## Usage

Run a Lox script:

```bash
cargo run -- script.lox
```

Start the REPL:

```bash
cargo run
```

## License

MIT License - see [LICENSE](LICENSE) for details.

This project is based on the Lox language specification from [Crafting Interpreters](https://craftinginterpreters.com/).
