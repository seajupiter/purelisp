use std::collections::HashMap;

use crate::ast::{Env, Value};
use crate::parse::parse;

pub fn load_prelude(env: &mut Env) {
    // List constructor function
    env.set(
        "list".to_string(),
        Value::Func(|args| {
            Value::List(args)
        }),
    );
    
    // car - get the first element of a list
    env.set(
        "car".to_string(),
        Value::Func(|args| {
            if args.len() != 1 {
                panic!("car requires exactly one argument");
            }
            match &args[0] {
                Value::List(list) => {
                    if list.is_empty() {
                        panic!("Cannot call car on an empty list");
                    }
                    list[0].clone()
                },
                _ => panic!("car requires a list argument"),
            }
        }),
    );
    
    // cdr - get all but the first element of a list
    env.set(
        "cdr".to_string(),
        Value::Func(|args| {
            if args.len() != 1 {
                panic!("cdr requires exactly one argument");
            }
            match &args[0] {
                Value::List(list) => {
                    if list.is_empty() {
                        panic!("Cannot call cdr on an empty list");
                    }
                    Value::List(list[1..].to_vec())
                },
                _ => panic!("cdr requires a list argument"),
            }
        }),
    );
    
    // cons - prepend an element to a list
    env.set(
        "cons".to_string(),
        Value::Func(|args| {
            if args.len() != 2 {
                panic!("cons requires exactly two arguments");
            }
            match &args[1] {
                Value::List(list) => {
                    let mut new_list = vec![args[0].clone()];
                    new_list.extend(list.clone());
                    Value::List(new_list)
                },
                _ => panic!("Second argument to cons must be a list"),
            }
        }),
    );
    
    // length - get the length of a list
    env.set(
        "length".to_string(),
        Value::Func(|args| {
            if args.len() != 1 {
                panic!("length requires exactly one argument");
            }
            match &args[0] {
                Value::List(list) => Value::Int(list.len() as i64),
                _ => panic!("length requires a list argument"),
            }
        }),
    );
    
    // nth - get the nth element of a list (0-indexed)
    env.set(
        "nth".to_string(),
        Value::Func(|args| {
            if args.len() != 2 {
                panic!("nth requires exactly two arguments");
            }
            let index = match &args[0] {
                Value::Int(i) => *i as usize,
                _ => panic!("First argument to nth must be an integer"),
            };
            match &args[1] {
                Value::List(list) => {
                    if index >= list.len() {
                        panic!("Index out of bounds in nth");
                    }
                    list[index].clone()
                },
                _ => panic!("Second argument to nth must be a list"),
            }
        }),
    );
    
    // append - concatenate two lists
    env.set(
        "append".to_string(),
        Value::Func(|args| {
            if args.len() != 2 {
                panic!("append requires exactly two arguments");
            }
            match (&args[0], &args[1]) {
                (Value::List(list1), Value::List(list2)) => {
                    let mut result = list1.clone();
                    result.extend(list2.clone());
                    Value::List(result)
                },
                _ => panic!("Both arguments to append must be lists"),
            }
        }),
    );
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
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_+ x y)"),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "-".to_string(),
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_- x y)"),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "*".to_string(),
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_* x y)"),
            mappings: HashMap::new(),
        },
    );
    env.set(
        "/".to_string(),
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_/ x y)"),
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
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_= x y)"),
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
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_<= x y)"),
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
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_< x y)"),
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
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_> x y)"),
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
        Value::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: parse("(_>= x y)"),
            mappings: HashMap::new(),
        },
    );
}