mod ast;
mod intpt;
mod parse;

use lalrpop_util::lalrpop_mod;
use std::env;
use std::path::Path;
lalrpop_mod!(pub purelisp);

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If a file path is provided, interpret the file
        let file_path = &args[1];
        let path = Path::new(file_path);

        if path.exists() {
            let mut env = intpt::create_environment();
            match intpt::file::process_file(path, &mut env) {
                Ok(res) => {
                    println!("File processed successfully with result: ");
                    for value in res {
                        println!("{:?}", value);
                    }
                }
                Err(e) => println!("Error processing file: {}", e),
            }
        } else {
            println!("File not found: {}", file_path);
        }
    } else {
        // Otherwise, start the REPL
        if intpt::repl::repl().is_err() {
            println!("REPL failed");
        }
    }
}
