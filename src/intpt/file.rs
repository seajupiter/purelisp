use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::ast::{Env, Expr, Value};
use crate::intpt::eval::eval;
use crate::parse;

/// Processes a multiline Lisp file
pub fn process_file<P: AsRef<Path>>(file_path: P, env: &mut Env) -> io::Result<Vec<Value>> {
    // Read the entire file contents
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Process the file contents
    process_string(&contents, env)
}

/// Processes a multiline string containing Lisp expressions
pub fn process_string(content: &str, env: &mut Env) -> io::Result<Vec<Value>> {
    let mut results = Vec::new();
    let mut buffer = String::new();
    let mut paren_count = 0;
    
    // Process the content line by line
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        
        // Count parentheses to determine if an expression is complete
        for c in trimmed.chars() {
            if c == '(' {
                paren_count += 1;
            } else if c == ')' {
                paren_count -= 1;
            }
        }
        
        // Append the current line to the buffer
        buffer.push_str(trimmed);
        buffer.push(' ');
        
        // If we have a complete expression, evaluate it
        if paren_count == 0 && !buffer.trim().is_empty() {
            let expr = parse::parse(&buffer);
            
            match expr {
                Expr::Def { x, y } => {
                    let value = eval(*y.clone(), env.clone());
                    env.set(x, value.clone());
                    results.push(value);
                },
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
                },
                _ => {
                    let value = eval(expr, env.clone());
                    results.push(value);
                }
            }
            
            // Reset the buffer and paren count for the next expression
            buffer.clear();
            paren_count = 0;
        }
    }
    
    // If there's still an unprocessed expression, try to evaluate it
    if paren_count != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unbalanced parentheses in file",
        ));
    } else if !buffer.trim().is_empty() {
        let expr = parse::parse(&buffer);
        let value = eval(expr, env.clone());
        results.push(value);
    }
    
    Ok(results)
}