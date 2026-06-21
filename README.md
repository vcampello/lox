# Lox

A work-in-progress Rust implementation of the Lox programming language from [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Goals

To implement the following:

- Interpreter - in progress
- Formatter
- LSP
- Debugger
- MCP server into each of the above

## Language Features

Currently implemented:

- Variables and assignment
- Blocks and lexical scoping
- Expressions:
    - binary operators
    - unary operators
    - logical operators (`and`, `or`)
- Comments
- `print` statement
- Control flow:
    - if
    - else
    - while loops
    - for loops
    - continue
    - break
- Literals:
    - numbers
    - strings
    - booleans
    - nil

Not yet implemented:

- Functions
- Classes

## Usage

Run a Lox script:

```bash
cargo run -- script.lox
```

Start the REPL:

```bash
cargo run

```

## Examples

**Variables and scoping:**

```lox
var a = "global";
{
  var a = "local";
  print a;  // local
}
print a;  // global
```

**For loops with Fibonacci:**

```lox
var a = 0;
var temp;

for (var b = 1; a < 100; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
```

**While loop with break/continue:**

```lox
var i = 0;
while (i < 10) {
  i = i + 1;

  if (i == 3) continue;
  if (i == 7) break;

  print i;  // prints 1, 2, 4, 5, 6
}
```

**Logical operators:**

```lox
print "hi" or 2;      // hi
print nil or "yes";   // yes
print true and "ok";  // ok
```

## License

MIT License - see [LICENSE](LICENSE) for details.

This project is based on the Lox language specification from [Crafting Interpreters](https://craftinginterpreters.com/).  
Tests adapted from the Crafting Interpreters test suite — Copyright (c) 2015 Robert Nystrom, MIT License.

