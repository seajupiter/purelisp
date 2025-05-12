# Lisp-in-Rust (MiniLisp)

An interpreter and compiler for a lisp-style functional programming language implemented in Rust, created as a project for the PKU Compiler Principles (Honor Track) course.

## Overview

## Features

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/lisp-in-rust.git
   cd lisp-in-rust
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Usage

Run the REPL:
```
cargo run
```

To execute a Lisp file:
```
cargo run -- path/to/your/file.lisp
```

## REPL Commands

- Exit the REPL with `(exit)` or by pressing `Ctrl+D`
- Execute standard Lisp expressions directly in the REPL

## Project Structure

- `src/ast.rs`: Abstract Syntax Tree definitions
- `src/parse.rs`: Parsing utilities and implementation
- `src/repl.rs`: REPL implementation
- `src/minilisp.lalrpop`: LALRPOP grammar for the Lisp dialect
- `src/main.rs`: Entry point and initialization

## Development

### Building with Features

Enable file history for the REPL:
```
cargo build --features with-file-history
```

### Running Tests

```
cargo test
```

## Future Improvements
