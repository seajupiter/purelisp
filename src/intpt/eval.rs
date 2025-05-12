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
                    // Identify all identifiers in the function body
                    let mut free_vars = std::collections::HashMap::new();

                    // Helper function to extract free variables from expressions
                    fn collect_free_vars(
                        expr: &Expr,
                        args: &Vec<String>,
                        free_vars: &mut std::collections::HashMap<String, Value>,
                        env: &Env,
                    ) {
                        match expr {
                            Expr::Id(id) if !args.contains(id) => {
                                if !free_vars.contains_key(id) {
                                    if let Some(value) = env.get(id) {
                                        free_vars.insert(id.clone(), value.clone());
                                    } else {
                                        panic!("Undefined identifier in closure: {}", id);
                                    }
                                }
                            }
                            Expr::Let { bindings, body } => {
                                // Let introduces new bindings, so we don't capture those
                                let binding_names: Vec<String> =
                                    bindings.iter().map(|(name, _)| name.clone()).collect();

                                // Process the body, but with binding names excluded
                                let mut temp_args = args.clone();
                                temp_args.extend(binding_names);
                                collect_free_vars(body, &temp_args, free_vars, env);

                                // Process binding expressions
                                for (_, expr) in bindings {
                                    collect_free_vars(expr, args, free_vars, env);
                                }
                            }
                            Expr::If { cond, then, else_ } => {
                                collect_free_vars(cond, args, free_vars, env);
                                collect_free_vars(then, args, free_vars, env);
                                collect_free_vars(else_, args, free_vars, env);
                            }
                            Expr::Fn {
                                args: inner_args,
                                body: inner_body,
                            } => {
                                // Function introduces new arguments, exclude them from captured variables
                                let mut temp_args = args.clone();
                                temp_args.extend(inner_args.clone());
                                collect_free_vars(inner_body, &temp_args, free_vars, env);
                            }
                            Expr::Def { x: _, y: value } => {
                                // Process the value expression
                                collect_free_vars(value, args, free_vars, env);
                            }
                            Expr::List(exprs) => {
                                for expr in exprs {
                                    collect_free_vars(expr, args, free_vars, env);
                                }
                            }
                            // For other expression types, no free variables to capture
                            _ => {}
                        }
                    }

                    collect_free_vars(&body, &args, &mut free_vars, &env);
                    free_vars
                },
            }
        }
        Expr::List(list) if !list.is_empty() => {
            let vals: Vec<Value> = list.iter().map(|e| eval(e.clone(), env.clone())).collect();
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
                    // println!("Calling closure {:?} on args {:?}", closure, list)
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
                        let remaining_params = params.iter().skip(args.len()).cloned().collect();

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
        Expr::List(_) => {
            panic!("Empty list");
        }
        Expr::Def { x: _, y: _ } => {
            panic!("Def expression only allowed in top level list");
        }
        Expr::Defun {
            name: _,
            args: _,
            body: _,
        } => {
            panic!("Defun expression only allowed in top level list");
        }
    }
}