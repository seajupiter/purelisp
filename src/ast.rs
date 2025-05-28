use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Id(String),
    Form(Vec<Expr>),
    Let {
        bindings: Vec<(String, Expr)>,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    And(Vec<Expr>),
    Or(Vec<Expr>),
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
    LetFun {
        name: String,
        args: Vec<String>,
        fun_body: Box<Expr>,
        expr_body: Box<Expr>,
    },
    DefClos {
        name: String,
        freevars: Vec<String>,
        args: Vec<String>,
        body: Box<Expr>,
    },
    Clos {
        name: String,
        mappings: HashMap<String, Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Value>),
    Func(fn(Vec<Value>) -> Value),
    Closure {
        params: Vec<String>,
        body: Expr,
        mappings: HashMap<String, Value>,
    },
}

// Display implementation for Expr
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Nil => write!(f, "nil"),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Int(i) => write!(f, "{}", i),
            Expr::Float(fl) => write!(f, "{}", fl),
            Expr::Str(s) => write!(f, "{}", s),
            Expr::Id(id) => write!(f, "{}", id),
            Expr::Form(list) => {
                write!(f, "(")?;
                for (i, e) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            Expr::Let { bindings, body } => {
                write!(f, "(let (")?;
                for (i, (id, e)) in bindings.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "({} {})", id, e)?;
                }
                write!(f, ") {})", body)
            }
            Expr::If { cond, then, else_ } => {
                write!(f, "(if {} {} {})", cond, then, else_)
            }
            Expr::Fn { args, body } => {
                write!(f, "(fn (")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ") {})", body)
            }
            Expr::And(exprs) => {
                write!(f, "(and")?;
                for e in exprs {
                    write!(f, " {}", e)?;
                }
                write!(f, ")")
            }
            Expr::Or(exprs) => {
                write!(f, "(or")?;
                for e in exprs {
                    write!(f, " {}", e)?;
                }
                write!(f, ")")
            }
            Expr::Def { x, y } => {
                write!(f, "(def {} {})", x, y)
            }
            Expr::Defun { name, args, body } => {
                write!(f, "(defun {} (", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ") {})", body)
            }
            Expr::LetFun {
                name,
                args,
                fun_body,
                expr_body,
            } => {
                write!(f, "(letfun ({} (", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ") {}) {})", fun_body, expr_body)
            }
            Expr::DefClos {
                name,
                freevars,
                args,
                body,
            } => {
                write!(f, "(defclos {} (", name)?;
                for (i, var) in freevars.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", var)?;
                }
                write!(f, ") (")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ") {})", body)
            }
            Expr::Clos { name, mappings } => {
                write!(f, "(clos {} (", name)?;
                for (i, (key, value)) in mappings.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "))")
            }
        }
    }
}

// Add display implementation for Values
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Str(s) => write!(f, "{}", s),
            Value::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Func(_) => write!(f, "<function>"),
            Value::Closure { params, .. } => write!(f, "<closure:{}>", params.join(" ")),
        }
    }
}


