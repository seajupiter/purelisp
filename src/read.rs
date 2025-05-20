use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::ast::Expr;
use crate::parse;

/// Reads a multiline string and parses it into a vector of expressions
pub fn read_string(content: &str) -> io::Result<Vec<Expr>> {
    let mut expressions = Vec::new();
    let mut buffer = String::new();
    let mut paren_count = 0;

    // Process the content line by line
    for line in content.lines() {
        let mut trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Process comments
        if let Some(comment_pos) = trimmed.find(';') {
            let code_part = trimmed[..comment_pos].trim();
            if code_part.is_empty() {
                continue; // The line is only a comment
            }
            trimmed = code_part;
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

        // If we have a complete expression, parse it
        if paren_count == 0 && !buffer.trim().is_empty() {
            let expr = parse::parse(&buffer);
            expressions.push(expr);

            // Reset the buffer and paren count for the next expression
            buffer.clear();
            paren_count = 0;
        }
    }

    // Check for unbalanced parentheses
    if paren_count != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unbalanced parentheses in content",
        ));
    }
    
    // Handle any remaining content in the buffer
    if !buffer.trim().is_empty() {
        let expr = parse::parse(&buffer);
        expressions.push(expr);
    }

    Ok(expressions)
}

/// Reads a file and parses it into a vector of expressions
pub fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<Expr>> {
    // Read the entire file contents
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Process the file contents
    read_string(&contents)
}
