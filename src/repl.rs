pub mod eval;
pub mod prelude;

use prelude::load_prelude;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::ast::{Env, Expr};

use crate::parse;

pub fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut env = Env::new();
    load_prelude(&mut env);

    loop {
        let readline = rl.readline("minilisp> ");
        match readline {
            Ok(line) => {
                #[cfg(feature = "with-file-history")]
                {
                    rl.add_history_entry(line.as_str())?;
                    rl.save_history("history.txt")?;
                }

                let expr = parse::parse(&line);
                println!("Parsed form: {:?}", expr);
                parse::print_expr(&expr);
                println!("");
                if let Expr::Def { x, y } = expr {
                    let value = eval::eval(*y.clone(), env.clone());
                    println!("Evaluation result of {:?}: {:?}", y, value);
                    env.set(x, value);
                } else {
                    let value = eval::eval(expr, env.clone());
                    println!("Evaluation result: {:?}", value);
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
