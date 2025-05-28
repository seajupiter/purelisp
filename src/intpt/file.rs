use std::io;
use std::path::Path;

use crate::ast::{Expr, Value};
use crate::intpt::Env;
use crate::intpt::eval::eval;
// We import read functions from the crate root

/// Processes a multiline Lisp file
pub fn process_file<P: AsRef<Path>>(file_path: P, env: &mut Env) -> io::Result<Vec<Value>> {
    // Read and parse expressions from the file
    let expressions = crate::read_file(file_path)?;

    // Evaluate all expressions
    evaluate_expressions(expressions, env)
}

/// Processes a multiline string containing Lisp expressions
pub fn process_string(content: &str, env: &mut Env) -> io::Result<Vec<Value>> {
    // Read and parse expressions from the string
    let expressions = crate::read_string(content)?;

    // Evaluate all expressions
    evaluate_expressions(expressions, env)
}

/// Helper function to evaluate a vector of expressions
fn evaluate_expressions(expressions: Vec<Expr>, env: &mut Env) -> io::Result<Vec<Value>> {
    let mut results = Vec::new();

    for expr in expressions {
        match expr {
            Expr::Def { x, y } => {
                let value = eval(*y.clone(), env.clone());
                env.set(x, value.clone());
                results.push(value);
            }
            Expr::Defun { name, args, body } => {
                // Create a closure for the function
                let closure = Value::Closure {
                    params: args,
                    body: *body,
                    mappings: std::collections::HashMap::new(),
                };
                // Bind the function name to the closure
                env.set(name, closure.clone());
                results.push(closure);
            }
            _ => {
                let value = eval(expr, env.clone());
                results.push(value);
            }
        }
    }

    Ok(results)
}
