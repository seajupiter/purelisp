mod eval;
mod prelude;

use prelude::load_prelude;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::ast::Env;

use crate::parse;

pub fn repl() -> Result<()>{
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
                rl.add_history_entry(line.as_str())?;

                let expr = parse::parse(line);
                if expr.is_none() {
                    println!("Parse failed");
                    continue;
                }
                let expr = expr.unwrap(); // Safe to unwrap since we checked for None above
                println!("Parsed form: {:?}", expr);
                parse::print_expr(&expr);
                println!("");
                let value = eval::eval(expr, env.clone());
                println!("Evaluation result: {:?}", value);
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

    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt")?;

    Ok(())
}