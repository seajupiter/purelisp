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
        Expr::If { cond, then, else_ } => {
            let new_then = Box::new(a_normal(*then, Box::new(|e| e)));
            let new_else = Box::new(a_normal(*else_, Box::new(|e| e)));
            cont(Expr::If {
                cond,
                then: new_then,
                else_: new_else,
            })
        }
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
        Expr::And(_) | Expr::Or(_) | Expr::Not(_) | Expr::DefClos { .. } | Expr::LetClos { .. } => {
            panic!("Invalid Expr for A-normalization: {}", expr)
        }
    }
}

pub fn a_normalize(prog: Vec<Expr>) -> Vec<Expr> {
    prog.iter()
        .map(|expr| a_normal(expr.clone(), Box::new(|e| e)))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{
        compl::{knormal::k_normalize, util::NameGenerator},
        pretty_format, read_string,
    };

    use super::*;

    #[test]
    fn anormal_test_nested_let() {
        let prog = read_string("(let ((x (let ((z 1) (w 2)) (* z w))) (y 2)) (+ x y))").unwrap();
        let kprog = k_normalize(prog.clone(), &mut NameGenerator::new());
        let aprog = a_normalize(kprog.clone());
        println!("original: {}", pretty_format(&prog[0]));
        println!("k-normalized: {}", pretty_format(&kprog[0]));
        println!("a-normalized: {}", pretty_format(&aprog[0]));
    }

    #[test]
    fn anormal_test_if() {
        let prog = read_string("(if (= 1 2) nil (let ((x (let ((y 1)) y))) x))").unwrap();
        let kprog = k_normalize(prog.clone(), &mut NameGenerator::new());
        let aprog = a_normalize(kprog.clone());
        println!("original: {}", pretty_format(&prog[0]));
        println!("k-normalized: {}", pretty_format(&kprog[0]));
        println!("a-normalized: {}", pretty_format(&aprog[0]));
    }

    #[test]
    fn anormal_test_letfun() {
        let prog = read_string("(letfun (f (x) x) (f (+ 1 (* 2 3))))").unwrap();
        let kprog = k_normalize(prog.clone(), &mut NameGenerator::new());
        let aprog = a_normalize(kprog.clone());
        println!("original: {}", pretty_format(&prog[0]));
        println!("k-normalized: {}", pretty_format(&kprog[0]));
        println!("a-normalized: {}", pretty_format(&aprog[0]));
    }

    #[test]
    fn anormal_test_fn() {
        let prog = read_string("(let ((f (fn (x) (+ (* x x) x)))) (f (+ 1 (* 2 3))))").unwrap();
        let kprog = k_normalize(prog.clone(), &mut NameGenerator::new());
        let aprog = a_normalize(kprog.clone());
        println!("original: {}", pretty_format(&prog[0]));
        println!("k-normalized: {}", pretty_format(&kprog[0]));
        println!("a-normalized: {}", pretty_format(&aprog[0]));
    }
}
