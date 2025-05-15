# PureLisp

A minimalist Lisp interpreter and compiler implemented in Rust, created as a project for the PKU Compiler Principles (Honor Track) course.

## Overview

PureLisp is a simple yet powerful Lisp-like language with both an interpreter and a compiler. The language supports key functional programming features like closures, higher-order functions, recursion, and more.

## Features

- Interactive REPL for quick experimentation
- File-based evaluation for larger programs
- Compiler for bytecode generation


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
   cargo build
   ```

### Usage

```
cargo run (-- <optional arguments>)
```

This basic command will start the PureLisp REPL (Read-Eval-Print Loop), where you can enter PureLisp expressions interactively.
It can take the following command-line options:
- `-l` or `--load`: Load a PureLisp file and start the REPL
- `--history`: turn on the history loading/saving feature

You can also directly evaluate a purelisp source code file by running:
```
cargo run -- <filename>
```

## Project Structure

The PureLisp project is organized with a clean separation of concerns:

- `src/ast.rs`: Defines the abstract syntax tree and environment structures
- `src/parse.rs`: Handles parsing of PureLisp code into AST
- `src/purelisp.lalrpop`: LALRPOP grammar definition for the language
- `src/intpt/(mod.rs)`: Interpreter implementation
  - `eval.rs`: Core evaluation logic
  - `prelude/`: Built-in functions and operators
  - `repl.rs`: Read-Eval-Print Loop implementation
  - `file.rs`: File processing utilities
- `src/compl/(mod.rs)`: Compiler implementation
  - `kform.rs`: K-normal form IR definitions
  - `transform.rs`: AST to K-normal form transformation
  - `codegen.rs`: Bytecode generation from K-normal form
- `examples/`: Example PureLisp programs demonstrating language features

## Syntax Specification

PureLisp has a minimalist Lisp syntax with several core expression types and built-in functions. Here's a comprehensive guide to the language syntax:

### Basic Types

```
nil                 ; Nil value
true false          ; Boolean values
42                  ; Integer
3.14                ; Float
"hello world"       ; String
identifier          ; Variable identifiers
(1 2 3)             ; List
```

### Function Calls

```
(function arg1 arg2 ...)  ; Function application
```

### Special Forms

#### Let Expressions
Creates local variable bindings:

```
(let (
      (var1 expr1)
      (var2 expr2)
      ...)
  body-expr)
```

#### If Expressions
Conditional branching:

```
(if condition
    then-expr
    else-expr)
```

#### Logical Operations
Short-circuit logical operations:

```
(and expr1 expr2 ...)  ; Returns true if all expressions are true
(or expr1 expr2 ...)   ; Returns true if any expression is true
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
(letfun (function-name (arg1 arg2 ...)
          function-body)
  expr-body)
```

#### Top-level Definitions
Global variable definition (only allowed at the top level):

```
(def variable-name expression)
```

#### Top-level (Recursive) Function Definitions
Global function definition (only allowed at the top level):

```
(defun function-name (arg1 arg2 ...)
  function-body)
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

#### Function Composition
```
; Example of function composition
(let ((add5 (fn (x) (+ x 5)))
      (mul2 (fn (x) (* x 2))))
  (mul2 (add5 3)))  ; Returns 16
```

#### Partial Application
```
; Example of partial application
(let ((add (fn (x y) (+ x y)))
      (add5 (add 5)))
  (add5 10))        ; Returns 15
```

#### List Operations
```
(list 1 2 3)                ; Creates a list (1 2 3)
(car (list 1 2 3))          ; Returns 1 (first element)
(cdr (list 1 2 3))          ; Returns (2 3) (rest of the list)
(cons 0 (list 1 2 3))       ; Returns (0 1 2 3) (prepends an element)
(length (list 1 2 3))       ; Returns 3 (length of the list)
(nth 1 (list 1 2 3))        ; Returns 2 (0-indexed element access)
(append (list 1 2) (list 3)) ; Returns (1 2 3) (concatenates lists)
```

#### Comments
```
; Single-line comments start with a semicolon
(+ 2 3) ; This adds 2 and 3
```

## Language Semantics

### Evaluation Model

PureLisp follows a strict evaluation strategy (eager evaluation), where arguments to functions are evaluated before the function is applied. This is in contrast to lazy evaluation, where expressions are only evaluated when their values are needed.

### Scoping Rules

PureLisp uses lexical scoping, where the scope of a variable is determined by the structure of the source code:

- Variables are bound in the closest enclosing `let`, `letfun`, or function parameters.
- Closures capture variables from their defining environment.
- Variable shadowing is allowed (inner bindings with the same name as outer bindings).

### Function Semantics

Functions in PureLisp are first-class values:

- Functions can be passed as arguments to other functions.
- Functions can be returned as results from functions.
- Functions can be stored in data structures.
- Closures automatically capture references to variables from their defining environment.
- Partial application is supported (calling a function with fewer arguments than it expects returns a new function).

### Recursion

PureLisp supports recursion through the `letfun` special form, which binds a function that can call itself. Recursive functions defined with `letfun` have proper lexical scoping, allowing them to reference variables from their enclosing environment.

## Examples

Please take a look at the `examples/` directory for various PureLisp programs demonstrating the language features.
