pub mod eval;
pub mod file;
pub mod prelude;
pub mod repl;

use std::collections::HashMap;
use crate::ast::Value;
use prelude::load_prelude;

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

    // pub fn unset(&mut self, key: &str) {
    //     if let Some(table) = self.tables.last_mut() {
    //         table.remove(key);
    //     }
    // }

    pub fn get(&self, key: &str) -> Option<&Value> {
        for table in self.tables.iter().rev() {
            if let Some(value) = table.get(key) {
                return Some(value);
            }
        }
        None
    }

    // pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
    //     for table in self.tables.iter_mut().rev() {
    //         if let Some(value) = table.get_mut(key) {
    //             return Some(value);
    //         }
    //     }
    //     None
    // }

    pub fn push(&mut self, map: HashMap<String, Value>) {
        self.tables.push(map);
    }

    // pub fn pop(&mut self) {
    //     if self.tables.len() > 1 {
    //         self.tables.pop();
    //     }
    // }
}

pub fn create_environment() -> Env {
    let mut env = Env::new();
    load_prelude(&mut env);
    env
}

// Re-export the REPL functions for convenient access
pub use repl::{repl, start_repl_with_env};