use std::panic;

use crate::ast::{Env, Expr, Value};

pub fn eval(expr: Expr, env: Env) -> Value {
    // println!("Evaluating: {:?}", expr);
    // println!("    with Environment: {:?}", env);
    match expr {
        Expr::Nil => Value::Nil,
        Expr::Bool(b) => Value::Bool(b.clone()),
        Expr::Int(i) => Value::Int(i.clone()),
        Expr::Float(f) => Value::Float(f.clone()),
        Expr::Str(s) => Value::Str(s.clone()),
        Expr::Id(id) => {
            if let Some(e) = env.get(&id) {
                e.clone()
            } else {
                panic!("Undefined identifier: {}", id);
            }
        }
        Expr::List(mut list) => {
            if let Some(first) = list.pop() {
                let f = eval(first.clone(), env.clone());
                match f {
                    Value::Func(func) => {
                        let args = list
                            .into_iter()
                            .map(|x| eval(x, env.clone()))
                            .rev()
                            .collect();
                        // println!("Calling function {:?} on args {:?}", f, args);
                        func(args)
                    }
                    Value::Closure {mut params, body, mut mappings} => {
                        // println!("Calling closure {:?} on args {:?}", closure, list);
                        while !params.is_empty() && !list.is_empty() {
                            mappings.insert(
                                params.pop().unwrap(),
                                eval(list.pop().unwrap(), env.clone()),
                            );
                        }
                        if !list.is_empty() {
                            panic!("Too many arguments for function {:?}", Value::Closure { params, body, mappings});
                        }
                        if params.is_empty() {
                            let mut newenv = env.clone();
                            newenv.push(mappings);
                            eval(body.clone(), newenv)
                        } else {
                            Value::Closure{
                                params,
                                body: body.clone(),
                                mappings: mappings.clone(),
                            }
                        }
                    }
                    _ => panic!("Type error: {:?}", f),
                }
            } else {
                panic!("Empty list");
            }
        },
        Expr::Let { bindings, body } => {
            todo!("Let bindings not implemented yet");
        }, 
        Expr::If { cond, then, else_ } => {
            todo!("If statement not implemented yet");
        }
    }
}
