# PureLisp

A minimalist Lisp interpreter implemented in Rust, created as a project for the PKU Compiler Principles (Honor Track) course.

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

The PureLisp project is organized with a clean separation of concerns:

- `src/ast.rs`: Defines the abstract syntax tree and environment structures
- `src/parse.rs`: Handles parsing of PureLisp code into AST
- `src/purelisp.lalrpop`: LALRPOP grammar definition for the language
- `src/intpt/`: Interpreter implementation
  - `eval.rs`: Core evaluation logic
  - `prelude.rs`: Built-in functions and operators
  - `repl.rs`: Read-Eval-Print Loop implementation
  - `file.rs`: File processing utilities
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

### Examples

Factorial function using recursion:
```
(letfun (factorial (n)
         (if (= n 0)
             1
             (* n (factorial (- n 1)))))
  (factorial 5))  ; Returns 120
```

Nested function definitions:
```
(letfun (outer (x)
         (letfun (inner (y) (+ x y))
           (inner (+ x 1))))
  (outer 5))  ; Returns 11
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

### List Processing Example

Here's an example demonstrating the use of lists and list processing functions:

```
; Create a list of numbers
(def numbers (list 1 2 3 4 5))

; Sum all elements in a list recursively
(letfun (sum-list (lst)
         (if (= (length lst) 0)
             0
             (+ (car lst) (sum-list (cdr lst)))))
  (sum-list numbers))  ; Returns 15

; Map a function over a list to create a new list
(letfun (map (f lst)
         (if (= (length lst) 0)
             (list)
             (cons (f (car lst)) (map f (cdr lst)))))
  (map (fn (x) (* x 2)) numbers))  ; Returns (2 4 6 8 10)

; Filter elements from a list
(letfun (filter (pred lst)
         (if (= (length lst) 0)
             (list)
             (if (pred (car lst))
                 (cons (car lst) (filter pred (cdr lst)))
                 (filter pred (cdr lst)))))
  (filter (fn (x) (> x 2)) numbers))  ; Returns (3 4 5)
```

This example demonstrates:
- Creating and manipulating lists
- Implementing higher-order functions for lists (map, filter)
- Recursive list processing
- Common list operations (car, cdr, cons, length)

### Comprehensive Example

Here's a more comprehensive example showing various PureLisp features:

```
; Define a higher-order function that applies a function n times
(letfun (apply-n-times (f n x)
          (if (= n 0)
              x
              (f (apply-n-times f (- n 1) x))))

  ; Define a list of operations to perform
  (let ((double (fn (x) (* x 2)))
        (add3 (fn (x) (+ x 3)))
        (square (fn (x) (* x x))))

    ; Demonstrate function composition
    (let ((result1 (apply-n-times double 3 2))     ; 2 -> 4 -> 8 -> 16
          (result2 (apply-n-times add3 2 5))       ; 5 -> 8 -> 11
          (result3 (apply-n-times square 2 3)))    ; 3 -> 9 -> 81

      ; Create a combined operation using function composition
      (let ((combined-op (fn (x)
                           (double (square (add3 x))))))

        ; Compare direct application with apply-n-times
        (let ((direct-result (combined-op 4))                ; 4 -> 7 -> 49 -> 98
              (composed-result (apply-n-times combined-op 2 4))) ; 4 -> 98 -> 203

          ; Return a list of all results
          (list result1 result2 result3 direct-result composed-result))))))
```

This example demonstrates:
- Higher-order functions (functions that take functions as arguments)
- Anonymous functions with `fn`
- Lexical closures
- Function composition
- Recursive function definitions with `letfun`
- Nested `let` expressions for creating local bindings

## Future Improvements
