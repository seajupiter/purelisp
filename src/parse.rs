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
                    } else if id == "if" && transformed_form.len() >= 4 {
                        // Transform if expression
                        let cond = Box::new(transformed_form[1].clone());
                        let then = Box::new(transformed_form[2].clone());
                        let else_ = Box::new(transformed_form[3].clone());

                        return Expr::If { cond, then, else_ };
                    } else if id == "fn" && transformed_form.len() >= 3 {
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
    match purelisp::ExprParser::new().parse(&input) {
        Ok(expr) => advance_parse(expr),
        Err(_) => panic!("Parse error"),
    }
}

pub fn print_expr(expr: &Expr) {
    match expr {
        Expr::Nil => print!("nil"),
        Expr::Bool(b) => print!("{}", b),
        Expr::Int(i) => print!("{}", i),
        Expr::Float(f) => print!("{}", f),
        Expr::Str(s) => print!("{}", s),
        Expr::Id(id) => print!("{}", id),
        Expr::Form(list) => {
            print!("(");
            for (i, e) in list.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print_expr(e);
            }
            print!(")");
        }
        Expr::Let { bindings, body } => {
            print!("(let (");
            for (i, (id, e)) in bindings.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("({} ", id);
                print_expr(e);
                print!(")");
            }
            print!(") ");
            print_expr(body);
            print!(")");
        }
        Expr::If { cond, then, else_ } => {
            print!("(if ");
            print_expr(cond);
            print!(" ");
            print_expr(then);
            print!(" ");
            print_expr(else_);
            print!(")");
        }
        Expr::Fn { args, body } => {
            print!("(fn (");
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", arg);
            }
            print!(") ");
            print_expr(body);
            print!(")");
        }
        Expr::Def { x, y } => {
            print!("(def {} ", x);
            print_expr(y);
            print!(")");
        }
        Expr::Defun { name, args, body } => {
            print!("(defun {} (", name);
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", arg);
            }
            print!(") ");
            print_expr(body);
            print!(")");
        }
        Expr::LetFun {
            name,
            args,
            fun_body,
            expr_body,
        } => {
            print!("(letfun ({} (", name);
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", arg);
            }
            print!(") ");
            print_expr(fun_body);
            print!(") ");
            print_expr(expr_body);
            print!(")");
        }
    }
}
