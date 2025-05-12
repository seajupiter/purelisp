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
        // Check if we should load a file before starting the REPL
        if args[1] == "-l" || args[1] == "--load" {
            if args.len() > 2 {
                // Load the file and then start the REPL
                let file_path = &args[2];
                let path = Path::new(file_path);

                if path.exists() {
                    let mut env = intpt::create_environment();
                    match intpt::file::process_file(path, &mut env) {
                        Ok(_) => {
                            println!("File loaded successfully.");
                            // Start REPL with the existing environment
                            if intpt::start_repl_with_env(env).is_err() {
                                println!("REPL failed");
                            }
                        }
                        Err(e) => println!("Error loading file: {}", e),
                    }
                } else {
                    println!("File not found: {}", file_path);
                    // Start REPL with a fresh environment
                    if intpt::repl().is_err() {
                        println!("REPL failed");
                    }
                }
            } else {
                println!("Missing file path after -l/--load");
                println!("Usage: purelisp [-l|--load FILE]");
                // Start REPL with a fresh environment
                if intpt::repl().is_err() {
                    println!("REPL failed");
                }
            }
        } else {
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
        }
    } else {
        // Otherwise, start the REPL
        if intpt::repl().is_err() {
            println!("REPL failed");
        }
    }
}
