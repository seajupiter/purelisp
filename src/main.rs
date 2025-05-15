use purelisp::intpt;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut use_history = false;
    let mut next_args = Vec::new();

    // First pass: extract global flags like --history
    for arg in args.iter().skip(1) {
        if arg == "--history" {
            use_history = true;
        } else if arg == "--help" {
            print_usage();
            return;
        } else {
            next_args.push(arg);
        }
    }

    if !next_args.is_empty() {
        if next_args[0] == "-l" || next_args[0] == "--load" {
            if next_args.len() > 1 {
                // Load the file and then start the REPL
                let file_path = &next_args[1];
                let path = Path::new(file_path);

                if path.exists() {
                    let mut env = intpt::create_environment();
                    match intpt::file::process_file(path, &mut env) {
                        Ok(_) => {
                            println!("File loaded successfully.");
                            // Start REPL with the existing environment
                            if intpt::start_repl_with_env(env, use_history).is_err() {
                                println!("REPL failed");
                            }
                        }
                        Err(e) => println!("Error loading file: {}", e),
                    }
                } else {
                    println!("File not found: {}", file_path);
                    // Start REPL with a fresh environment
                    if intpt::repl(use_history).is_err() {
                        println!("REPL failed");
                    }
                }
            } else {
                println!("Missing file path after -l/--load");
                print_usage();
                // Start REPL with a fresh environment
                if intpt::repl(use_history).is_err() {
                    println!("REPL failed");
                }
            }
        } else {
            // If a file path is provided, interpret the file
            let file_path = &next_args[0];
            let path = Path::new(file_path);

            if path.exists() {
                let mut env = intpt::create_environment();
                match intpt::file::process_file(path, &mut env) {
                    Ok(res) => {
                        println!("File processed successfully with result: ");
                        for value in res {
                            println!("{}", value);
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
        if intpt::repl(use_history).is_err() {
            println!("REPL failed");
        }
    }
}

fn print_usage() {
    println!("PureLisp - A minimalist Lisp interpreter and compiler");
    println!();
    println!("Usage:");
    println!("  purelisp                               Start the REPL");
    println!("  purelisp [--history] FILE              Execute FILE");
    println!("  purelisp [--history] -l|--load FILE    Execute FILE then start the REPL");
    println!();
    println!("Options:");
    println!("  -h, --help      Show this help message");
    println!("  --history       Enable REPL history");
    println!("  -l, --load      Load and execute a file before starting the REPL");
    println!("  compile, --compile, -c     Compile a file to bytecode (default output is FILE.plb)");
}
