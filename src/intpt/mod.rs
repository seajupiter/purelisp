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