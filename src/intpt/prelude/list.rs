use crate::ast::{Env, Value};

pub fn load_list(env: &mut Env) {
    // List constructor function
    env.set("list".to_string(), Value::Func(|args| Value::List(args)));

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
                }
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
                }
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
                }
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
                }
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
                }
                _ => panic!("Both arguments to append must be lists"),
            }
        }),
    );
}
