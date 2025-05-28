use crate::Expr;

use super::util::NameGenerator;

fn is_atom(expr: &Expr) -> bool {
    match expr {
        Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) | Expr::Id(_) => {
            true
        }
        _ => false,
    }
}

fn k_normal(expr: Expr, namer: &mut NameGenerator) -> Expr {
    match expr {
        Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) | Expr::Id(_) => {
            expr
        }
        Expr::Form(form) => {
            let mut kform: Vec<Expr> = form.iter().map(|e| k_normal(e.clone(), namer)).collect();
            let mut bindings = Vec::new();
            for e in kform.iter_mut() {
                if !is_atom(e) {
                    let temp = namer.next();
                    bindings.push((temp.clone(), e.clone()));
                    *e = Expr::Id(temp.clone());
                }
            }
            if bindings.is_empty() {
                Expr::Form(kform)
            } else {
                Expr::Let {
                    bindings,
                    body: Box::new(Expr::Form(kform)),
                }
            }
        }
        Expr::Let { bindings, body } => {
            let mut new_bindings = Vec::new();
            for (name, expr) in bindings {
                new_bindings.push((name.clone(), k_normal(expr, namer)));
            }
            Expr::Let {
                bindings: new_bindings,
                body: Box::new(k_normal(*body, namer)),
            }
        }
        Expr::If { cond, then, else_ } => {
            let cond = k_normal(*cond, namer);
            let then = k_normal(*then, namer);
            let else_ = k_normal(*else_, namer);
            if is_atom(&cond) {
                Expr::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    else_: Box::new(else_),
                }
            } else {
                let temp = namer.next();
                let let_binding = Expr::Let {
                    bindings: vec![(temp.clone(), cond)],
                    body: Box::new(Expr::If {
                        cond: Box::new(Expr::Id(temp)),
                        then: Box::new(then),
                        else_: Box::new(else_),
                    }),
                };
                k_normal(let_binding, namer)
            }
        }
        Expr::And(exprs) => {
            // Transform (and a b c) into (if a (if b c false) false)
            // then k-normalize the resulting if expression
            if exprs.is_empty() {
                Expr::Bool(true)
            } else {
                let mut result = exprs.last().unwrap().clone();
                for expr in exprs.into_iter().rev().skip(1) {
                    result = Expr::If {
                        cond: Box::new(expr),
                        then: Box::new(result),
                        else_: Box::new(Expr::Bool(false)),
                    };
                }
                k_normal(result, namer)
            }
        }
        Expr::Or(exprs) => {
            // Transform (or a b c) into (if a true (if b true c))
            // then k-normalize the resulting if expression
            if exprs.is_empty() {
                Expr::Bool(false)
            } else {
                let mut result = exprs.last().unwrap().clone();
                for expr in exprs.into_iter().rev().skip(1) {
                    result = Expr::If {
                        cond: Box::new(expr),
                        then: Box::new(Expr::Bool(true)),
                        else_: Box::new(result),
                    };
                }
                k_normal(result, namer)
            }
        }
        Expr::Fn { args, body } => {
            // Normalize the function body
            Expr::Fn {
                args,
                body: Box::new(k_normal(*body, namer)),
            }
        }
        Expr::Def { x, y } => {
            // Normalize the definition's value
            Expr::Def {
                x,
                y: Box::new(k_normal(*y, namer)),
            }
        }
        Expr::Defun { name, args, body } => {
            // Normalize the function body
            Expr::Defun {
                name,
                args,
                body: Box::new(k_normal(*body, namer)),
            }
        }
        Expr::LetFun {
            name,
            args,
            fun_body,
            expr_body,
        } => {
            // Normalize both function body and expression body
            Expr::LetFun {
                name,
                args,
                fun_body: Box::new(k_normal(*fun_body, namer)),
                expr_body: Box::new(k_normal(*expr_body, namer)),
            }
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

fn expand_let(expr: Expr) -> Expr {
    match expr {
        Expr::Let { bindings, body } => {
            let mut new_body = *body;
            for (name, expr) in bindings.into_iter().rev() {
                new_body = Expr::Let {
                    bindings: vec![(name, expr)],
                    body: Box::new(new_body),
                };
            }
            new_body
        }
        _ => expr,
    }
}

pub fn k_normalize(expr: Expr, namer: &mut NameGenerator) -> Expr {
    let k_normal_expr = k_normal(expr, namer);
    expand_let(k_normal_expr)
}

#[cfg(test)]
mod test {
    use crate::{parse, pretty_format};

    use super::*;

    #[test]
    fn knormal_test_funcall() {
        let expr = parse("(+ (+ 1 2) (* 3 4))");
        let kexpr = k_normalize(expr.clone(), &mut NameGenerator::new("t"));
        println!("original: {}", pretty_format(&expr));
        println!("k-normalized: {}", pretty_format(&kexpr));
    }
    #[test]
    fn knormal_test_if() {
        let expr = parse("(if (> 1 2) (+ 3 4) (* 5 6))");
        let kexpr = k_normalize(expr.clone(), &mut NameGenerator::new("t"));
        println!("original: {}", pretty_format(&expr));
        println!("k-normalized: {}", pretty_format(&kexpr));
    }
    #[test]
    fn knormal_test_and() {
        let expr = parse("(and (> 1 2) (< 3 4))");
        let kexpr = k_normalize(expr.clone(), &mut NameGenerator::new("t"));
        println!("original: {}", pretty_format(&expr));
        println!("k-normalized: {}", pretty_format(&kexpr));
    }
    #[test]
    fn knormal_test_or() {
        let expr = parse("(or (> 1 2) (< 3 4))");
        let kexpr = k_normalize(expr.clone(), &mut NameGenerator::new("t"));
        println!("original: {}", pretty_format(&expr));
        println!("k-normalized: {}", pretty_format(&kexpr));
    }
    #[test]
    fn knormal_test_letfun() {
        let expr = parse("(letfun (f (x) (+ (* x x) x)) (f (+ 1 (* 2 3))))");
        let kexpr = k_normalize(expr.clone(), &mut NameGenerator::new("t"));
        println!("original: {}", pretty_format(&expr));
        println!("k-normalized: {}", pretty_format(&kexpr));
    }
}
