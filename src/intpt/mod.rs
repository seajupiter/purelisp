pub mod eval;
pub mod file;
pub mod prelude;
pub mod repl;

use crate::ast::Env;
use prelude::load_prelude;

pub fn create_environment() -> Env {
    let mut env = Env::new();
    load_prelude(&mut env);
    env
}

// Re-export the REPL functions for convenient access
pub use repl::{repl, start_repl_with_env};