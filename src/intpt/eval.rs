use std::collections::HashMap;
use std::panic;

use crate::ast::{Expr, Value};
use crate::intpt::Env;

// Helper function to extract free variables from expressions
fn collect_free_vars(
    expr: &Expr,
    args: &Vec<String>,
    func_name: Option<&String>,
    free_vars: &mut HashMap<String, Value>,
    env: &Env,
) {
    match expr {
        Expr::Id(id) => {
            if !args.contains(id) && func_name.map_or(true, |name| id != name) {
                if !free_vars.contains_key(id) {
                    if let Some(value) = env.get(id) {
                        free_vars.insert(id.clone(), value.clone());
                    } else {
                        panic!("Undefined identifier in closure: {}", id);
                    }
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
            collect_free_vars(body, &temp_args, func_name, free_vars, env);

            // Process binding expressions
            for (_, expr) in bindings {
                collect_free_vars(expr, args, func_name, free_vars, env);
            }
        }
        Expr::If { cond, then, else_ } => {
            collect_free_vars(cond, args, func_name, free_vars, env);
            collect_free_vars(then, args, func_name, free_vars, env);
            collect_free_vars(else_, args, func_name, free_vars, env);
        }
        Expr::Fn {
            args: inner_args,
            body: inner_body,
        } => {
            // Function introduces new arguments, exclude them from captured variables
            let mut temp_args = args.clone();
            temp_args.extend(inner_args.clone());
            collect_free_vars(inner_body, &temp_args, func_name, free_vars, env);
        }
        Expr::LetFun {
            name: inner_name,
            args: inner_args,
            fun_body: inner_fun_body,
            expr_body: inner_expr_body,
        } => {
            // For nested letfun, we need to handle inner function separately

            // For the function body, exclude both the function name and its args
            let mut temp_args = args.clone();
            temp_args.extend(inner_args.clone());
            temp_args.push(inner_name.clone());

            // Process the function body with both function args excluded
            collect_free_vars(inner_fun_body, &temp_args, func_name, free_vars, env);

            // For the expression body, only exclude the function name
            // (the inner function should be able to use variables from the outer scope)
            let mut expr_args = args.clone();
            expr_args.push(inner_name.clone());

            // Process the expression body with function name excluded
            collect_free_vars(inner_expr_body, &expr_args, func_name, free_vars, env);
        }
        Expr::Def { x: _, y: value } => {
            // Process the value expression
            collect_free_vars(value, args, func_name, free_vars, env);
        }
        Expr::Form(exprs) => {
            for expr in exprs {
                collect_free_vars(expr, args, func_name, free_vars, env);
            }
        }
        Expr::And(exprs) => {
            for expr in exprs {
                collect_free_vars(expr, args, func_name, free_vars, env);
            }
        }
        Expr::Or(exprs) => {
            for expr in exprs {
                collect_free_vars(expr, args, func_name, free_vars, env);
            }
        }
        _ => {}
    }
}

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
                    let mut free_vars = HashMap::new();
                    collect_free_vars(&body, &args, None, &mut free_vars, &env);
                    free_vars
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
        Expr::Def { x: _, y: _ } => {
            panic!("Def expression only allowed in top level form");
        }
        Expr::Defun {
            name: _,
            args: _,
            body: _,
        } => {
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
                    let mut mappings = HashMap::new();
                    collect_free_vars(&fun_body, &args, Some(&name), &mut mappings, &env);
                    mappings
                },
            };

            let mut new_mappings = HashMap::new();
            new_mappings.insert(name.clone(), closure);
            new_env.push(new_mappings);

            // Evaluate the body with the function defined
            eval(*expr_body, new_env)
        }
        Expr::DefClos {
            name: _,
            freevars: _,
            args: _,
            body: _,
        } => {
            panic!("DefClos not allowed");
        }
        Expr::Clos {
            name: _,
            mappings: _,
        } => {
            panic!("Clos not allowed");
        }
    }
}
