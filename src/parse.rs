use crate::ast::Expr;
use crate::mal;

pub fn parse(input: String) -> Option<Expr> {
    match mal::ExprParser::new().parse(&input) {
        Ok(expr) => Some(expr),
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
            for (i, e) in list.iter().rev().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print_expr(e);
            }
            print!(")");
        }
        Expr::Let { bindings, body } => {
            print!("(let ");
            for (i, (id, e)) in bindings.iter().rev().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{} ", id);
                print_expr(e);
            }
            print!(" ");
            for e in body.iter().rev() {
                print_expr(e);
            }
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
