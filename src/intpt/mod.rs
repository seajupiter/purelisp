pub mod eval;
pub mod file;
pub mod prelude;
pub mod repl;

use crate::ast::Value;
use prelude::load_prelude;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    tables: Vec<HashMap<String, Value>>,
}

impl Env {
    pub fn init(&mut self) {
        self.tables.push(HashMap::new());
    }

    pub fn new() -> Self {
        let mut env = Env { tables: Vec::new() };
        env.init();
        env
    }

    pub fn set(&mut self, key: String, value: Value) {
        let table = self.tables.last_mut().unwrap();
        table.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        for table in self.tables.iter().rev() {
            if let Some(value) = table.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn push(&mut self, map: HashMap<String, Value>) {
        self.tables.push(map);
    }
}

pub fn create_environment() -> Env {
    let mut env = Env::new();
    load_prelude(&mut env);
    env
}

// Re-export the REPL functions for convenient access
pub use repl::{repl, start_repl_with_env};
