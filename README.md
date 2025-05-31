# PureLisp

A minimalist Lisp-like pure functional language with an interpreter and a compiler implemented in Rust, created as a project for the PKU Compiler Principles (Honor Track) course.

## Overview

PureLisp is a simple yet powerful Lisp-like dynamically-typed language with both an interpreter and a compiler (to C code). The language supports key functional programming features such as first-class functions and more.

## Feature Roadmap

- [x] Interactive REPL for quick experimentation
- [x] File-based evaluation for larger programs
- [x] Compiler for C-code generation
- [x] Higher-order functions
- [x] Function Currying (partial evaluation) support for interpreter (the compiler does not support it yet)
- [ ] Lisp quote/unquote syntax
- [ ] Pairs, list datatype support
- [ ] Garbage collection


## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/purelisp/purelisp.git
   cd purelisp
   ```

2. Install the binary:
   ```
   cargo install --path .
   ```
   Or you can use `cargo run` to run the project directly.

### Usage

```
purelisp [--history]                              Start the REPL
purelisp FILE                                     Execute FILE
purelisp [--history] -l|--load FILE               Execute FILE then start the REPL
purelisp compile [-ir] FILE [-o OUTPUT]           Compile FILE to C-code (default) or PureLisp IR

Options:
  -h, --help                Show this help message
  --history                 Enable REPL history
  -l, --load                Load and execute a file before starting the REPL
  -ir                       Compile a file to PureLisp IR (default is C-code)
  -o, --output FILE         Specify output file for compilation (default is INPUT.plir/c)
```

## Main Project Structure

- `src/main.rs`: Main procedure for the PureLisp interpreter and compiler binary
- `src/lib.rs`: Library entry point for PureLisp
- `src/purelisp.lalrpop`: LALRPOP grammar definition for the language
- `src/ast.rs`: the abstract syntax tree
- `src/parse.rs`: parse and obtain a single expression
- `src/read.rs`: utilities to read and parse PureLisp source code into a PureLisp program (a sequence of expressions)
- `src/formatter.rs`: a simple formatter prettify a PureLisp program
- `src/intpt/(mod.rs)`: Interpreter implementation
  - `eval.rs`: Core evaluation logic
  - `prelude/`: Built-in functions and operators
  - `repl.rs`: Read-Eval-Print Loop implementation
  - `file.rs`: File interpreting implementation
- `src/compl/(mod.rs)`: Compiler implementation
  - `knormal.rs`: K-normalization
  - `anormal.rs`: A-normalization
  - `copyprop.rs`: Copy propagation optimization
  - `closure.rs`: Closure conversion
  - `codegen.rs`: C-code generation
  - `runtime.rs`: C runtime for PureLisp

## Syntax Specification

PureLisp has a minimalist Lisp-like syntax with several core expression types and built-in functions. Here's a comprehensive guide to the language syntax:

### Basic Expressions

```
nil                 ; Nil value
true false          ; Boolean values
42                  ; Integer
3.14                ; Float
"hello world"       ; String literal
x                   ; Variable identifiers
```

### Function Calls

```
(<function> <arg1> <arg2> ...)  ; function application
```

### Special Forms

#### Let Expressions
Creates local variable bindings:

```
(let (
      (<var1> <expr1>)
      (<var2> <expr2>)
      ...)
  <body-expr>)
```

#### If Expressions
Conditional branching:

```
(if <condition>
    <then-expr>
    <else-expr>)
```

#### Logical Operations
Short-circuit logical operations:

```
(and <expr1> <expr2> ...)  ; Returns true if all expressions are true
(or <expr1> <expr2> ...)   ; Returns true if any expression is true
(not <expr>)             ; Logical negation
```

#### Lambda Functions
Anonymous function creation:

```
(fn (arg1 arg2 ...)
    body-expr)
```

#### Recursive Functions
Create recursive function bindings:

```
(letfun (<func-name> (<arg1> <arg2> ...)
          <func-body>)
  <expr-body>)
```

#### Top-level Definitions
Global variable definition (only allowed at the top level):

```
(def <var> <expr>)
```

#### Top-level (Recursive) Function Definitions
Global function definition (only allowed at the top level):

```
(defun <func-name> (<arg1> <arg2> ...)
  <func-body>)
```

### Built-in Functions and Operators

PureLisp provides several built-in functions for common operations:

#### Arithmetic Operators
```
(+ x y)              ; Addition
(- x y)              ; Subtraction
(* x y)              ; Multiplication
(/ x y)              ; Division
```

#### Comparison Operators
```
(= x y)              ; Equality check
(< x y)              ; Less than
(<= x y)             ; Less than or equal
(> x y)              ; Greater than
(>= x y)             ; Greater than or equal
```

### Comments
```
; Single-line comments start with a semicolon
(+ 2 3) ; End-of-line comment
```

## Language Semantics

### Evaluation Model

PureLisp follows a strict evaluation strategy (eager evaluation), where arguments to functions are evaluated before the function is applied. This is in contrast to lazy evaluation, where expressions are only evaluated when their values are needed.

### Scoping Rules

PureLisp uses lexical scoping, where the scope of a variable is determined by the structure of the source code:

- Variables are bound in the closest enclosing `let`, `letfun`, or function parameters.
- Closures capture variables from their defining environment.
- Variable shadowing is allowed (inner bindings with the same name as outer bindings).
- In compiling mode, global definitions (using `def` or `defun`) are available throughout the program, regardless of where they are defined.

### Function Semantics

Functions in PureLisp are first-class values:

- Functions can be passed as arguments to other functions.
- Functions can be returned as results from functions.
- Functions can be stored in data structures.
- Closures automatically capture references to variables from their defining environment.
- Partial application is supported in interpreting mode (calling a function with fewer arguments than it expects returns a new function).

## Examples

Please take a look at the `examples/` directory for various PureLisp programs demonstrating the language features.
