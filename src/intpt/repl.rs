use rustyline::{DefaultEditor, Result};

use crate::ast::{Env, Expr, Value};
use crate::intpt::eval::eval;
use crate::parse;
use rustyline::error::ReadlineError;
use std::collections::HashMap;

pub fn repl(use_history: bool) -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    if use_history {
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
    }

    let env = crate::intpt::create_environment();
    start_session(&mut rl, env, use_history)
}

/// Start the REPL with an existing environment
/// Useful for when a file is loaded before the REPL starts
pub fn start_repl_with_env(env: Env, use_history: bool) -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    if use_history {
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
    }

    start_session(&mut rl, env, use_history)
}

fn start_session(rl: &mut DefaultEditor, mut env: Env, use_history: bool) -> Result<()> {
    loop {
        let readline = rl.readline("purelisp> ");
        match readline {
            Ok(line) => {
                if use_history {
                    rl.add_history_entry(line.as_str())?;
                    rl.save_history("history.txt")?;
                }

                let expr = parse::parse(&line);
                // println!("Parsed form: {:?}", expr);
                // println!("{}", expr);
                // print!(" -> ");

                if let Expr::Def { x, y } = expr {
                    let value = eval(*y.clone(), env.clone());
                    println!("Evaluation result of {:?}: {:?}", y, value);
                    env.set(x, value);
                } else if let Expr::Defun { name, args, body } = expr {
                    // Create a closure for the function
                    let closure = Value::Closure {
                        params: args,
                        body: *body,
                        mappings: HashMap::new(),
                    };
                    // Bind the function name to the closure
                    env.set(name.clone(), closure);
                    println!("Function {} defined", name);
                } else {
                    let value = eval(expr, env.clone());
                    println!("{}", value);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
