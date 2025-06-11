use crate::Expr;

#[derive(Debug, Clone, PartialEq)]
struct Env(Vec<(String, Expr)>);
impl Env {
    fn new() -> Self {
        Env(Vec::new())
    }
    fn push(&mut self, x: String, e: Expr) {
        self.0.push((x, e));
    }
    fn get(&self, id: &str) -> Option<&Expr> {
        for (name, expr) in self.0.iter().rev() {
            if name == id {
                return Some(expr);
            }
        }
        None
    }
    fn get_origin(&self, id: &str) -> Expr {
        // println!("get_origin: {}", id);
        let mut id = id.to_string();
        loop {
            if let Some(expr) = self.get(&id) {
                match expr {
                    Expr::Id(new_id) => {
                        id = new_id.clone();
                    }
                    _ => return expr.clone(),
                }
            } else {
                return Expr::Id(id);
            }
        }
    }
}

pub fn copy_prop(prog: Vec<Expr>) -> Vec<Expr> {
    let mut env = Env::new();
    let mut new_prog = Vec::new();
    for expr in prog {
        match expr {
            Expr::Def { x, y } => {
                let new_y = copy_prop_helper(*y, &env);
                if new_y.is_atom() {
                    if new_y != Expr::Id(x.clone()) {
                        env.push(x.clone(), new_y.clone());
                    }
                } else {
                    new_prog.push(Expr::Def {
                        x,
                        y: Box::new(new_y),
                    });
                }
            }
            Expr::Defun { name, args, body } => {
                let new_body = copy_prop_helper(*body, &env);
                new_prog.push(Expr::Defun {
                    name,
                    args,
                    body: Box::new(new_body),
                });
            }
            _ => {
                let optimized_expr = copy_prop_helper(expr, &env);
                new_prog.push(optimized_expr);
            }
        }
    }
    new_prog
}

fn copy_prop_helper(expr: Expr, env: &Env) -> Expr {
    // println!("copy_prop_help: expr={}, env={:?}", expr, env);
    match expr {
        Expr::Def { .. }
        | Expr::Defun { .. }
        | Expr::DefClos { .. }
        | Expr::LetClos { .. }
        | Expr::Fn { .. }
        | Expr::And(_)
        | Expr::Or(_)
        | Expr::Not(_) => {
            panic!("Invalid Expr for copy_prop: {}", expr);
        }
        Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) => expr,
        Expr::Id(id) => env.get_origin(&id),
        Expr::Let { bindings, body } => {
            assert!(
                bindings.len() == 1,
                "Copy propagation only supports single binding let (needs to be A-normalized)"
            );

            let (x, e) = bindings[0].clone();
            if e == Expr::Id(x.clone()) {
                // If the binding is a direct copy of itself, we can optimize it away
                *body
            } else {
                let optimized_e = copy_prop_helper(e.clone(), env);

                if optimized_e.is_atom() {
                    let mut new_env = env.clone();
                    new_env.push(x.clone(), optimized_e.clone());
                    copy_prop_helper(*body, &new_env)
                } else {
                    let optimized_body = copy_prop_helper(*body, env);
                    Expr::Let {
                        bindings: vec![(x, optimized_e)],
                        body: Box::new(optimized_body),
                    }
                }
            }
        }
        Expr::If { cond, then, else_ } => {
            let optimized_cond = copy_prop_helper(*cond, env);
            let optimized_then = copy_prop_helper(*then, env);
            let optimized_else = copy_prop_helper(*else_, env);

            Expr::If {
                cond: Box::new(optimized_cond),
                then: Box::new(optimized_then),
                else_: Box::new(optimized_else),
            }
        }
        Expr::LetFun {
            name,
            args,
            fun_body,
            expr_body,
        } => {
            let optimized_fun_body = copy_prop_helper(*fun_body, env);
            let optimized_expr_body = copy_prop_helper(*expr_body, env);

            Expr::LetFun {
                name,
                args,
                fun_body: Box::new(optimized_fun_body),
                expr_body: Box::new(optimized_expr_body),
            }
        }
        Expr::Form(exprs) => {
            let optimized_exprs = exprs
                .into_iter()
                .map(|e| copy_prop_helper(e, env))
                .collect();

            Expr::Form(optimized_exprs)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        compl::{anormal::a_normalize, closure::closure_convert, knormal::k_normalize}, format_prog, read_file
    };

    use super::*;

    #[test]
    fn test_copyprop() {
        let prog = read_file("./examples/factorial.purelisp")
            .unwrap();
        let mut namer = crate::compl::util::NameGenerator::new();
        let kprog = k_normalize(prog.clone(), &mut namer);
        let aprog = a_normalize(kprog.clone());
        let cprog = copy_prop(aprog.clone());
        let lprog = closure_convert(cprog.clone(), &mut namer);
        println!("original:\n{}", format_prog(&prog));
        println!("k-normalized:\n{}", format_prog(&kprog));
        println!("a-normalized:\n{}", format_prog(&aprog));
        println!("copy-propagated:\n{}", format_prog(&cprog));
        println!("closure-converted:\n{}", format_prog(&lprog));
    }
}
