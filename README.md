# Riku Programming Language

Riku is a simple interpreted programming language designed for educational purposes. This repository contains the source code for the Riku interpreter, which can execute Riku scripts from files or in an interactive command-line interface (CLI) mode.

## Features

- Basic arithmetic operations: addition, subtraction, multiplication, and division.
- Logical operations: and, or, not.
- Comparison operations: equal, not equal, greater than, greater than or equal, less than, less than or equal.
- Variable declaration and assignment.
- Conditional statements (`if`, `else`).
- Looping constructs (`while`).
- Input and output operations.
- Nested scopes with support for variable shadowing.

## Directory Structure

- `src/`
  - `env.rs`: Defines the environment for variable storage and scope management.
  - `error.rs`: Contains error handling utilities.
  - `expr.rs`: Defines the expression evaluation logic.
  - `lib.rs`: Entry point for the library, contains functions to run the interpreter in file or CLI mode.
  - `main.rs`: Entry point for the executable, handles command-line arguments.
  - `parser.rs`: Implements the parser for the Riku language.
  - `source.rs`: Tokenizes the input source code.
  - `stmt.rs`: Defines the statement evaluation logic.
  - `token.rs`: Defines the token types and token structure.

## Getting Started

### Prerequisites

- Rust programming language (https://www.rust-lang.org/)

### Building the Project

1. Clone the repository:

   ```sh
   git clone https://github.com/Sanu-2004/riku.git
   cd riku
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

### Running the Interpreter

You can run the Riku interpreter in two modes: file mode and CLI mode.

#### File Mode

To run a Riku script from a file, use the following command:

```sh
cargo run --release <source_file>
```

Replace `<source_file>` with the path to your Riku script.

#### CLI Mode

To start the interpreter in interactive CLI mode, simply run:

```sh
cargo run --release
```

In CLI mode, you can type and execute Riku commands directly. To exit the CLI, type `exit()`.

### Example

Here is an example Riku script:

```riku
let a = 1;
while a < 10 {
    print(a)
    a = a + 1
}
```

Save this script to a file (e.g., `example.riku`) and run it using the interpreter:

```sh
cargo run --release example.riku
```

## Language Syntax

### Variables

Variables can be declared using the `let` keyword and assigned values using the `=` operator.

```riku
let x = 10;
x = x + 5;
```

### Arithmetic Operations

Riku supports basic arithmetic operations: `+`, `-`, `*`, `/`.

```riku
let result = (5 + 3) * 2;
```

### Logical Operations

Riku supports logical operations: `&` (and), `|` (or), `!` (not).

```riku
let is_true = true & false;
```

### Comparison Operations

Riku supports comparison operations: `==`, `!=`, `>`, `>=`, `<`, `<=`.

```riku
let is_equal = 5 == 5;
```

### Conditional Statements

Riku supports `if` and `else` statements for conditional execution.

```riku
if x > 10 {
    print("x is greater than 10")
} else {
    print("x is not greater than 10")
}
```

### Loops

Riku supports `while` loops for repeated execution.

```riku
let i = 0;
while i < 5 {
    print(i)
    i = i + 1
}
```

### Input and Output

Riku supports `print` for output and `input` for input.

```riku
print("Enter your name: ")
let name = input()
print("Hello, " + name)
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

This project was inspired by various educational programming languages and interpreters. Special thanks to the Rust community for their excellent documentation and support.

---

Happy coding!
