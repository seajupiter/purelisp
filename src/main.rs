use purelisp::read_file;
use purelisp::{compl, intpt};

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut use_history = false;
    let mut next_args = Vec::new();
    let mut is_compile_mode = false;
    let mut is_compile_to_ir = false;
    let mut output_path = None;

    // First pass: extract global flags like --history
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--history" {
            use_history = true;
        } else if arg == "--help" || arg == "-h" {
            print_usage();
            return;
        } else if arg == "--compile" || arg == "-c" || arg == "compile" {
            is_compile_mode = true;
        } else if arg == "--compile-ir" || arg == "-ir" {
            is_compile_mode = true;
            is_compile_to_ir = true;
        } else if arg == "--output" || arg == "-o" {
            if i + 1 < args.len() {
                output_path = Some(args[i + 1].clone());
                i += 1;
            } else {
                println!("Missing output path after --output/-o");
                print_usage();
                return;
            }
        } else {
            next_args.push(arg);
        }
        i += 1;
    }

    if !next_args.is_empty() {
        if is_compile_mode {
            // Compile mode - compile the source file to binary or C code
            let source_path = &next_args[0];
            let path = Path::new(source_path);

            if path.exists() {
                // Determine output path if not explicitly specified
                let out_path = match output_path {
                    Some(p) => PathBuf::from(p),
                    None => {
                        let mut p = PathBuf::from(source_path);
                        if is_compile_to_ir {
                            p.set_extension("plir"); // purelisp IR
                        } else {
                            p.set_extension("c"); // C
                        }
                        p
                    }
                };

                match compile_file(path, &out_path, is_compile_to_ir) {
                    Ok(()) => println!(
                        "Successfully compiled {} to {}",
                        source_path,
                        out_path.display()
                    ),
                    Err(e) => println!("Error compiling file: {}", e),
                }
            } else {
                println!("Source file not found: {}", source_path);
            }
        } else if next_args[0] == "-l" || next_args[0] == "--load" {
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

#[rustfmt::skip]
fn print_usage() {
    println!(r##"
PureLisp - A minimalist Lisp-like pure-functional language interpreter and compiler

Usage:
  purelisp [--history]                              Start the REPL
  purelisp FILE                                     Execute FILE
  purelisp [--history] -l|--load FILE               Execute FILE then start the REPL
  purelisp compile [-ir] FILE [-o OUTPUT]           Compile FILE to C-code (default) or PureLisp IR

Options:
  -h, --help                Show this help message
  --history                 Enable REPL history
  -l, --load                Load and execute a file before starting the REPL
  -ir                       Compile a file to PureLisp IR (default is C-code)
  -o, --output FILE         Specify output file for compilation (default is INPUT.plir/c)
"##);
}

/// Compiles a source file to a binary file
fn compile_file<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    is_compile_to_ir: bool,
) -> io::Result<()> {
    let prog = read_file(input_path)?;
    // Compile the file
    let compiled_code = if is_compile_to_ir {
        compl::compl_to_ir(prog)
    } else {
        compl::compl_to_c(prog)
    };
    // Write the compiled code to the output file
    let mut file = fs::File::create(output_path)?;
    file.write_all(compiled_code.as_bytes())?;
    Ok(())
}
