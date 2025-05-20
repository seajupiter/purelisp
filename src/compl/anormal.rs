use crate::Expr;

pub fn a_normal(expr: Expr, cont: Box<dyn FnOnce(Expr) -> Expr>) -> Expr {
    match expr {
        Expr::Nil
        | Expr::Int(_)
        | Expr::Float(_)
        | Expr::Str(_)
        | Expr::Bool(_)
        | Expr::Id(_)
        | Expr::Form(_) => cont(expr),
        Expr::Let { bindings, body } => {
            assert!(
                bindings.len() == 1,
                "A-normalization only supports single binding let (have you k-normalized it in advance?)"
            );
            let (x, e) = &bindings[0];
            let (x, e) = (x.clone(), e.clone());
            let new_body = a_normal(*body, cont);
            a_normal(
                e.clone(),
                Box::new(move |c| Expr::Let {
                    bindings: vec![(x.clone(), c)],
                    body: Box::new(new_body),
                }),
            )
        }
        Expr::If {
            cond: _,
            then: _,
            else_: _,
        } => cont(expr),
        Expr::LetFun {
            name,
            args,
            fun_body,
            expr_body,
        } => {
            let new_fun_body = Box::new(a_normal(*fun_body, Box::new(|e| e)));
            let new_expr_body = Box::new(a_normal(*expr_body, cont));
            Expr::LetFun {
                name,
                args,
                fun_body: new_fun_body,
                expr_body: new_expr_body,
            }
        }
        Expr::Fn { args, body } => {
            let new_body = Box::new(a_normal(*body, Box::new(|e| e)));
            let new_expr = Expr::Fn {
                args,
                body: new_body,
            };
            cont(new_expr)
        }
        Expr::Def { x, y } => {
            let new_y = Box::new(a_normal(*y, Box::new(|e| e)));
            Expr::Def { x, y: new_y }
        }
        Expr::Defun { name, args, body } => {
            let new_body = Box::new(a_normal(*body, Box::new(|e| e)));
            Expr::Defun {
                name,
                args,
                body: new_body,
            }
        }
        Expr::And(_) | Expr::Or(_) => {
            panic!(
                "A-normalization does not support and/or expressions (have you k-normalized it in advance?)"
            )
        }
    }
}

pub fn a_normalize(expr: Expr) -> Expr {
    a_normal(expr, Box::new(|e| e))
}

#[cfg(test)]
mod test {
    use crate::{
        compl::{knormal::k_normalize, util::NameGenerator},
        parse, pretty_format,
    };

    use super::*;

    #[test]
    fn anormal_test_nested_let() {
        let expr = parse("(let ((x (let ((y 1)) y))) x)");
        let kexpr = k_normalize(expr.clone(), &mut NameGenerator::new("t"));
        let aexpr = a_normalize(kexpr.clone());
        println!("original: {}", pretty_format(&expr));
        println!("k-normalized: {}", pretty_format(&kexpr));
        println!("a-normalized: {}", pretty_format(&aexpr));
    }

    #[test]
    fn anormal_test_letfun() {
        let expr = parse("(letfun (f (x) x) (f (+ 1 (* 2 3))))");
        let kexpr = k_normalize(expr.clone(), &mut NameGenerator::new("t"));
        let aexpr = a_normalize(kexpr.clone());
        println!("original: {}", pretty_format(&expr));
        println!("k-normalized: {}", pretty_format(&kexpr));
        println!("a-normalized: {}", pretty_format(&aexpr));
    }
}
