# Purelisp

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
cargo run (--features with-file-history)
```

To evaluate a Purelisp file:
```
cargo run -- path/to/your/file.purelisp
```

To load a file and start the REPL: 
```
cargo run -- -l/--load path/to/your/file.purelisp
```

## Project Structure


## Future Improvements
