use crate::ast::Expr;
use crate::mal;

pub fn advance_parse(expr: Expr) -> Expr {
    match expr {
        Expr::List(list) => {
            // Transform each element in the list recursively
            let transformed_list: Vec<Expr> =
                list.iter().map(|e| advance_parse(e.clone())).collect();

            // Check if this is a special form
            if !transformed_list.is_empty() {
                if let Expr::Id(id) = &transformed_list[0] {
                    if id == "let" && transformed_list.len() == 3 {
                        // Transform let expression
                        let mut bindings = Vec::new();

                        // Check that the second element is a list of bindings
                        if let Expr::List(binding_list) = &transformed_list[1] {
                            // Process bindings (each binding is a 2-element list)
                            for binding in binding_list {
                                if let Expr::List(pair) = binding {
                                    if pair.len() == 2 {
                                        if let Expr::Id(var_name) = &pair[0] {
                                            bindings.push((var_name.clone(), pair[1].clone()));
                                        }
                                    }
                                }
                            }
                        }

                        // The 3rd element is the body expression
                        let body = Box::new(transformed_list[2].clone());

                        return Expr::Let { bindings, body };
                    } else if id == "if" && transformed_list.len() >= 4 {
                        // Transform if expression
                        let cond = Box::new(transformed_list[1].clone());
                        let then = Box::new(transformed_list[2].clone());
                        let else_ = Box::new(transformed_list[3].clone());

                        return Expr::If { cond, then, else_ };
                    }
                }
            }

            // If not a special form, return as a regular list
            Expr::List(transformed_list)
        }
        // For non-list expressions, return as is
        _ => expr,
    }
}

pub fn parse(input: String) -> Option<Expr> {
    match mal::ExprParser::new().parse(&input) {
        Ok(expr) => Some(advance_parse(expr)),
        Err(_) => None,
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
        Expr::List(list) => {
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
    }
}
