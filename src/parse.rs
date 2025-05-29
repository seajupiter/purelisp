use crate::ast::Expr;
use crate::purelisp;

fn advance_parse(expr: Expr) -> Expr {
    match expr {
        Expr::Form(form) => {
            // Transform each element in the form recursively
            let transformed_form: Vec<Expr> =
                form.iter().map(|e| advance_parse(e.clone())).collect();

            // Check if this is a special form
            if !transformed_form.is_empty() {
                if let Expr::Id(id) = &transformed_form[0] {
                    if id == "let" && transformed_form.len() == 3 {
                        // Transform let expression
                        let mut bindings = Vec::new();

                        // Check that the second element is a list of bindings
                        if let Expr::Form(binding_list) = &transformed_form[1] {
                            // Process bindings (each binding is a 2-element list)
                            for binding in binding_list {
                                if let Expr::Form(pair) = binding {
                                    if pair.len() == 2 {
                                        if let Expr::Id(var_name) = &pair[0] {
                                            bindings.push((var_name.clone(), pair[1].clone()));
                                        }
                                    }
                                }
                            }
                        }

                        // The 3rd element is the body expression
                        let body = Box::new(transformed_form[2].clone());

                        return Expr::Let { bindings, body };
                    } else if id == "if" && transformed_form.len() == 4 {
                        // Transform if expression
                        let cond = Box::new(transformed_form[1].clone());
                        let then = Box::new(transformed_form[2].clone());
                        let else_ = Box::new(transformed_form[3].clone());

                        return Expr::If { cond, then, else_ };
                    } else if id == "and" {
                        // Transform n-ary and expression
                        let exprs = transformed_form[1..].to_vec();
                        return Expr::And(exprs);
                    } else if id == "or" {
                        // Transform n-ary or expression
                        let exprs = transformed_form[1..].to_vec();
                        return Expr::Or(exprs);
                    } else if id == "not" && transformed_form.len() == 2 {
                        // Transform not expression
                        let expr = Box::new(transformed_form[1].clone());
                        return Expr::Not(expr);
                    } else if id == "fn" && transformed_form.len() == 3 {
                        // Transform fn (lambda function) expression
                        let mut args = Vec::new();

                        // Check that the second element is a list of argument names
                        if let Expr::Form(arg_list) = &transformed_form[1] {
                            // Extract argument names
                            for arg in arg_list {
                                if let Expr::Id(arg_name) = arg {
                                    args.push(arg_name.clone());
                                }
                            }
                        }

                        // The third element is the body expression
                        let body = Box::new(transformed_form[2].clone());

                        return Expr::Fn { args, body };
                    } else if id == "def" && transformed_form.len() == 3 {
                        // Transform def expression
                        if let Expr::Id(x) = &transformed_form[1] {
                            let y = Box::new(transformed_form[2].clone());
                            return Expr::Def { x: x.clone(), y };
                        } else {
                            panic!("First argument to def must be an identifier");
                        }
                    } else if id == "defun" && transformed_form.len() == 4 {
                        // Transform defun expression
                        if let Expr::Id(func_name) = &transformed_form[1] {
                            let mut args = Vec::new();

                            // Check that the third element is a list of argument names
                            if let Expr::Form(arg_list) = &transformed_form[2] {
                                // Extract argument names
                                for arg in arg_list {
                                    if let Expr::Id(arg_name) = arg {
                                        args.push(arg_name.clone());
                                    } else {
                                        panic!("Arguments to defun must be identifiers");
                                    }
                                }
                            } else {
                                panic!("Second argument to defun must be a list of argument names");
                            }

                            let body = Box::new(transformed_form[3].clone());

                            return Expr::Defun {
                                name: func_name.clone(),
                                args,
                                body,
                            };
                        } else {
                            panic!("First argument to defun must be an identifier");
                        }
                    } else if id == "letfun" && transformed_form.len() == 3 {
                        // Transform letfun expression
                        if let Expr::Form(func_def) = &transformed_form[1] {
                            if func_def.len() == 3 {
                                if let Expr::Id(func_name) = &func_def[0] {
                                    let mut args = Vec::new();

                                    // Check that the second element is a list of argument names
                                    if let Expr::Form(arg_list) = &func_def[1] {
                                        // Extract argument names
                                        for arg in arg_list {
                                            if let Expr::Id(arg_name) = arg {
                                                args.push(arg_name.clone());
                                            } else {
                                                panic!("Arguments to letfun must be identifiers");
                                            }
                                        }
                                    } else {
                                        panic!(
                                            "Second element in letfun function definition must be a list of argument names"
                                        );
                                    }

                                    let fun_body = Box::new(func_def[2].clone());
                                    let expr_body = Box::new(transformed_form[2].clone());

                                    return Expr::LetFun {
                                        name: func_name.clone(),
                                        args,
                                        fun_body,
                                        expr_body,
                                    };
                                } else {
                                    panic!(
                                        "First element in letfun function definition must be an identifier"
                                    );
                                }
                            } else {
                                panic!(
                                    "letfun function definition must have three elements: name, args list, and body"
                                );
                            }
                        } else {
                            panic!("First argument to letfun must be a function definition");
                        }
                    }
                }
            }

            // If not a special form, return as a regular form
            Expr::Form(transformed_form)
        }
        _ => expr,
    }
}

pub fn parse(input: &str) -> Expr {
    // println!("lalr parse: {}", input);
    match purelisp::ExprParser::new().parse(&input) {
        Ok(expr) => advance_parse(expr),
        Err(_) => panic!("Parse error"),
    }
}
