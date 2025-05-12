use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Id(String),
    List(Vec<Expr>),
    Let {
        bindings: Vec<(String, Expr)>,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Fn {
        args: Vec<String>,
        body: Box<Expr>,
    },
    Def {
        x: String,
        y: Box<Expr>,
    },
    Defun {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Func(fn(Vec<Value>) -> Value),
    Closure {
        params: Vec<String>,
        body: Expr,
        mappings: HashMap<String, Value>,
    },
}

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

    pub fn push(&mut self, map: HashMap<String, Value>) {
        self.tables.push(map);
    }

    // pub fn pop(&mut self) {
    //     if self.tables.len() > 1 {
    //         self.tables.pop();
    //     }
    // }
}
