use std::collections::HashMap;
use std::panic;

use crate::ast::{Expr, Value};
use crate::intpt::Env;

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
        Expr::And(exprs) => {
            // n-ary short-circuit AND
            for expr in exprs {
                let val = eval(expr.clone(), env.clone());
                match val {
                    Value::Bool(false) => return Value::Bool(false),
                    Value::Bool(true) => continue,
                    _ => panic!("All arguments to 'and' must be boolean"),
                }
            }
            Value::Bool(true)
        }
        Expr::Or(exprs) => {
            // n-ary short-circuit OR
            for expr in exprs {
                let val = eval(expr.clone(), env.clone());
                match val {
                    Value::Bool(true) => return Value::Bool(true),
                    Value::Bool(false) => continue,
                    _ => panic!("All arguments to 'or' must be boolean"),
                }
            }
            Value::Bool(false)
        }
        Expr::Not(expr) => {
            let val = eval(*expr, env.clone());
            match val {
                Value::Bool(b) => Value::Bool(!b),
                _ => panic!("Argument to 'not' must be boolean"),
            }
        }
        Expr::Let { bindings, body } => {
            // Create a new environment by extending the current one
            let mut new_env = env.clone();
            let mut new_mappings = std::collections::HashMap::new();

            // Evaluate each binding and add it to the new environment
            for (id, expr) in bindings {
                let value = eval(expr, new_env.clone());
                new_mappings.insert(id, value);
            }

            new_env.push(new_mappings);

            // Evaluate the body with the new environment
            eval(*body, new_env)
        }
        Expr::If { cond, then, else_ } => {
            let cond_val = eval(*cond, env.clone());
            match cond_val {
                Value::Bool(true) => eval(*then, env),
                Value::Bool(false) => eval(*else_, env),
                _ => panic!("If condition must evaluate to a boolean"),
            }
        }
        Expr::Fn { args, body } => {
            Value::Closure {
                params: args.clone(),
                body: *body.clone(),
                mappings: {
                    // Create bounded set with function arguments
                    let mut bounded = std::collections::HashSet::new();
                    for arg in &args {
                        bounded.insert(arg.clone());
                    }

                    // Get free variable names using the new method
                    let free_var_names = body.free_vars(&bounded);

                    // Build mappings from environment
                    let mut mappings = HashMap::new();
                    for var_name in free_var_names {
                        if let Some(value) = env.get(&var_name) {
                            mappings.insert(var_name, value.clone());
                        } else {
                            panic!("Undefined identifier in closure: {}", var_name);
                        }
                    }
                    mappings
                },
            }
        }
        Expr::Form(form) => {
            if form.is_empty() {
                panic!("Empty form");
            } else {
                let vals: Vec<Value> = form.iter().map(|e| eval(e.clone(), env.clone())).collect();
                let f = &vals[0];
                match f {
                    Value::Func(func) => {
                        let args = vals[1..].to_vec();
                        // println!("Calling function {:?} on args {:?}", f, args);
                        func(args)
                    }
                    Value::Closure {
                        params,
                        body,
                        mappings,
                    } => {
                        let args = vals[1..].to_vec();

                        if args.len() > params.len() {
                            panic!(
                                "Too many arguments for function {:?}",
                                Value::Closure {
                                    params: params.clone(),
                                    body: body.clone(),
                                    mappings: mappings.clone()
                                }
                            );
                        }

                        // Map arguments to parameters
                        let mut new_mappings = mappings.clone();
                        for (i, param) in params.iter().enumerate() {
                            if i < args.len() {
                                new_mappings.insert(param.clone(), args[i].clone());
                            } else {
                                break;
                            }
                        }

                        if args.len() == params.len() {
                            let mut newenv = env.clone();
                            newenv.push(new_mappings);
                            eval(body.clone(), newenv)
                        } else {
                            // Partial application
                            let remaining_params =
                                params.iter().skip(args.len()).cloned().collect();

                            Value::Closure {
                                params: remaining_params,
                                body: body.clone(),
                                mappings: new_mappings,
                            }
                        }
                    }
                    _ => panic!("Type error: {:?}", f),
                }
            }
        }
        Expr::Def { .. } => {
            panic!("Def expression only allowed in top level form");
        }
        Expr::Defun { .. } => {
            panic!("Defun expression only allowed in top level form");
        }
        Expr::LetFun {
            name,
            args,
            fun_body,
            expr_body,
        } => {
            // Create a new environment for the letfun expression
            let mut new_env = env.clone();

            // Create the function closure
            let closure = Value::Closure {
                params: args.clone(),
                body: *fun_body.clone(),
                mappings: {
                    // Create bounded set with function name and arguments
                    let mut bounded = std::collections::HashSet::new();
                    bounded.insert(name.clone()); // Function can refer to itself
                    for arg in &args {
                        bounded.insert(arg.clone());
                    }

                    // Get free variable names using the new method
                    let free_var_names = fun_body.free_vars(&bounded);

                    // Build mappings from environment
                    let mut mappings = HashMap::new();
                    for var_name in free_var_names {
                        if let Some(value) = env.get(&var_name) {
                            mappings.insert(var_name, value.clone());
                        } else {
                            panic!("Undefined identifier in closure: {}", var_name);
                        }
                    }
                    mappings
                },
            };

            let mut new_mappings = HashMap::new();
            new_mappings.insert(name.clone(), closure);
            new_env.push(new_mappings);

            // Evaluate the body with the function defined
            eval(*expr_body, new_env)
        }
        Expr::DefClos { .. } => {
            panic!("DefClos not allowed");
        }
        Expr::LetClos { .. } => {
            panic!("Clos not allowed");
        }
    }
}
