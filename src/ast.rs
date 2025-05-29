use std::{
    collections::{HashMap, HashSet},
    fmt,
};

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
    Not(Box<Expr>),
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
    LetClos {
        name: String,
        closid: String,
        freevars: Vec<String>,
        body: Box<Expr>,
    },
}

// Primitive symbols (builtin functions)
pub const PRIMITIVES: [&str; 16] = [
    "+", "-", "*", "/", "=", "<", "<=", ">", ">=", "list", "car", "cdr", "cons", "length", "nth",
    "append",
];

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
            Expr::Not(expr) => {
                write!(f, "(not {})", expr)
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
            Expr::LetClos {
                name,
                closid,
                freevars: mappings,
                body,
            } => {
                write!(f, "(letclos ({} {} (", name, closid)?;
                for (i, id) in mappings.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", id)?;
                }
                write!(f, ") {})", body)
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

impl Expr {
    pub fn is_atom(&self) -> bool {
        matches!(
            self,
            Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) | Expr::Id(_)
        )
    }

    /// Collect free variables in this expression, excluding any variables in `bounded`
    pub fn free_vars(&self, bounded: &HashSet<String>) -> Vec<String> {
        let mut free_vars = HashSet::new();
        let mut bounded = bounded.clone();
        for id in PRIMITIVES.iter() {
            bounded.insert(id.to_string());
        }
        self.collect_free_vars_helper(&bounded, &mut free_vars);
        let mut result: Vec<String> = free_vars.into_iter().collect();
        result.sort(); // For deterministic output
        // println!("free_vars: {:?}", result);
        result
    }

    /// Helper method to recursively collect free variables
    fn collect_free_vars_helper(&self, bounded: &HashSet<String>, free_vars: &mut HashSet<String>) {
        match self {
            Expr::Id(id) => {
                if !bounded.contains(id) {
                    free_vars.insert(id.clone());
                }
            }
            Expr::Let { bindings, body } => {
                // Process binding expressions first with current bounded variables
                for (_, expr) in bindings {
                    expr.collect_free_vars_helper(bounded, free_vars);
                }

                // Process the body with binding names added to bounded variables
                let mut new_bounded = bounded.clone();
                for (name, _) in bindings {
                    new_bounded.insert(name.clone());
                }
                body.collect_free_vars_helper(&new_bounded, free_vars);
            }
            Expr::If { cond, then, else_ } => {
                cond.collect_free_vars_helper(bounded, free_vars);
                then.collect_free_vars_helper(bounded, free_vars);
                else_.collect_free_vars_helper(bounded, free_vars);
            }
            Expr::Fn { args, body } => {
                // Function introduces new arguments, exclude them from free variables
                let mut new_bounded = bounded.clone();
                for arg in args {
                    new_bounded.insert(arg.clone());
                }
                body.collect_free_vars_helper(&new_bounded, free_vars);
            }
            Expr::LetFun {
                name,
                args,
                fun_body,
                expr_body,
            } => {
                // For the function body, exclude both the function name and its args
                let mut fun_bounded = bounded.clone();
                fun_bounded.insert(name.clone());
                for arg in args {
                    fun_bounded.insert(arg.clone());
                }
                fun_body.collect_free_vars_helper(&fun_bounded, free_vars);

                // For the expression body, only exclude the function name
                let mut expr_bounded = bounded.clone();
                expr_bounded.insert(name.clone());
                expr_body.collect_free_vars_helper(&expr_bounded, free_vars);
            }
            Expr::Def { .. } | Expr::Defun { .. } | Expr::DefClos { .. } => {
                panic!("Invalid Expr for free_var collection: {}", self);
            }
            Expr::Form(exprs) => {
                for expr in exprs {
                    expr.collect_free_vars_helper(bounded, free_vars);
                }
            }
            Expr::And(exprs) | Expr::Or(exprs) => {
                for expr in exprs {
                    expr.collect_free_vars_helper(bounded, free_vars);
                }
            }
            Expr::Not(expr) => {
                expr.collect_free_vars_helper(bounded, free_vars);
            }
            Expr::LetClos {
                name,
                closid: _,
                freevars: _,
                body,
            } => {
                let mut new_bounded = bounded.clone();
                new_bounded.insert(name.clone());
                body.collect_free_vars_helper(&new_bounded, free_vars);
            }
            // Atoms have no free variables
            Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) => {}
        }
    }
}
