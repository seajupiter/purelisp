mod ast;
mod parse;
mod repl; 

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mal);


fn main() {
    if repl::repl().is_err() {
        println!("REPL failed");
    }
}