use std::collections::HashMap;

use crate::ast::{Env, Expr, Value};

pub fn load_prelude(env: &mut Env) {
    env.set(
        "_+".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for addition");
            } else if args.len() != 2 {
                panic!("Addition requires two arguments");
            } else {
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                    _ => panic!("Type error in addition"),
                }
            }
        }),
    );
    env.set(
        "_-".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for subtraction");
            } else if args.len() != 2 {
                panic!("Subtraction requires two arguments");
            } else {
                println!("args: {:?}", args);
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
                    _ => panic!("Type error in subtraction"),
                }
            }
        }),
    );
    env.set(
        "_*".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for multiplication");
            } else if args.len() != 2 {
                panic!("Multiplication requires two arguments");
            } else {
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
                    _ => panic!("Type error in multiplication"),
                }
            }
        }),
    );
    env.set(
        "_/".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for division");
            } else if args.len() != 2 {
                panic!("Division requires two arguments");
            } else {
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
                    _ => panic!("Type error in division"),
                }
            }
        }),
    );
    env.set(
        "+".to_string(),
        Value::Closure {
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_+".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "-".to_string(),
        Value::Closure {
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_-".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "*".to_string(),
        Value::Closure {
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_*".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "/".to_string(),
        Value::Closure {
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_/".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "_=".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for equality");
            } else if args.len() != 2 {
                panic!("Equality requires two arguments");
            } else {
                Value::Bool(args[0] == args[1])
            }
        }),
    );
    env.set(
        "=".to_string(),
        Value::Closure{
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_=".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "_<=".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for less than or equal");
            } else if args.len() != 2 {
                panic!("Less than or equal requires two arguments");
            } else {
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Bool(x <= y),
                    (Value::Float(x), Value::Float(y)) => Value::Bool(x <= y),
                    _ => panic!("Type error in less than or equal"),
                }
            }
        }),
    );
    env.set(
        "<=".to_string(),
        Value::Closure{
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_<=".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "_<".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for less than");
            } else if args.len() != 2 {
                panic!("Less than requires two arguments");
            } else {
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Bool(x < y),
                    (Value::Float(x), Value::Float(y)) => Value::Bool(x < y),
                    _ => panic!("Type error in less than"),
                }
            }
        }),
    );
    env.set(
        "<".to_string(),
        Value::Closure{
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_<".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "_>".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for greater than");
            } else if args.len() != 2 {
                panic!("Greater than requires two arguments");
            } else {
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Bool(x > y),
                    (Value::Float(x), Value::Float(y)) => Value::Bool(x > y),
                    _ => panic!("Type error in greater than"),
                }
            }
        }),
    );
    env.set(
        ">".to_string(),
        Value::Closure{
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_>".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "_>=".to_string(),
        Value::Func(|args| {
            if args.is_empty() {
                panic!("No arguments for greater than or equal");
            } else if args.len() != 2 {
                panic!("Greater than or equal requires two arguments");
            } else {
                match (args[0].clone(), args[1].clone()) {
                    (Value::Int(x), Value::Int(y)) => Value::Bool(x >= y),
                    (Value::Float(x), Value::Float(y)) => Value::Bool(x >= y),
                    _ => panic!("Type error in greater than or equal"),
                }
            }
        }),
    );
    env.set(
        ">=".to_string(),
        Value::Closure{
            params: vec!["y".to_string(), "x".to_string()],
            body: Expr::List(vec![
                Expr::Id("y".to_string()),
                Expr::Id("x".to_string()),
                Expr::Id("_>=".to_string()),
            ]),
            mappings: HashMap::new(),
        },
    );
}
