use std::collections::HashSet;

use super::util::NameGenerator;
use crate::Expr;

struct ClosureConverter {
    global_defs: Vec<Expr>,
    freevars_exclude: HashSet<String>,
}

impl ClosureConverter {
    fn new() -> Self {
        ClosureConverter {
            global_defs: Vec::new(),
            freevars_exclude: HashSet::new(),
        }
    }
    fn id_map(expr: Expr, from: &str, to: &str) -> Expr {
        match expr {
            Expr::And(_)
            | Expr::Or(_)
            | Expr::Not(_)
            | Expr::Fn { .. }
            | Expr::Def { .. }
            | Expr::Defun { .. }
            | Expr::DefClos { .. } => {
                panic!("Expr not allowed: {}", expr);
            }
            Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) => expr,
            Expr::Id(id) => {
                if id == from {
                    Expr::Id(to.to_string())
                } else {
                    Expr::Id(id)
                }
            }
            Expr::Form(form) => {
                let new_form = form
                    .into_iter()
                    .map(|item| Self::id_map(item, from, to))
                    .collect();
                Expr::Form(new_form)
            }
            Expr::If { cond, then, else_ } => {
                let new_cond = Box::new(Self::id_map(*cond, from, to));
                let new_then = Box::new(Self::id_map(*then, from, to));
                let new_else = Box::new(Self::id_map(*else_, from, to));
                Expr::If {
                    cond: new_cond,
                    then: new_then,
                    else_: new_else,
                }
            }
            Expr::Let { bindings, body } => {
                let new_bindings = bindings
                    .into_iter()
                    .map(|(x, e)| (x, Self::id_map(e, from, to)))
                    .collect();
                let new_body = Box::new(Self::id_map(*body, from, to));
                Expr::Let {
                    bindings: new_bindings,
                    body: new_body,
                }
            }
            Expr::LetFun {
                name,
                args,
                fun_body,
                expr_body,
            } => {
                let new_fun_body = Box::new(Self::id_map(*fun_body, from, to));
                let new_expr_body = Box::new(Self::id_map(*expr_body, from, to));
                Expr::LetFun {
                    name,
                    args,
                    fun_body: new_fun_body,
                    expr_body: new_expr_body,
                }
            }
            Expr::LetClos {
                name,
                closid,
                freevars,
                body,
            } => {
                let new_body = Box::new(Self::id_map(*body, from, to));
                Expr::LetClos {
                    name,
                    closid,
                    freevars,
                    body: new_body,
                }
            }
        }
    }

    fn convert_helper(&mut self, expr: Expr, namer: &mut NameGenerator) -> Expr {
        match expr {
            Expr::And(_)
            | Expr::Or(_)
            | Expr::Not(_)
            | Expr::LetClos { .. }
            | Expr::DefClos { .. }
            | Expr::Fn { .. } => {
                panic!("Expr not allowed: {}", expr);
            }
            Expr::Nil
            | Expr::Bool(_)
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::Str(_)
            | Expr::Id(_) => expr,
            Expr::Form(form) => {
                let mut new_form = Vec::new();
                for item in form {
                    new_form.push(self.convert_helper(item, namer));
                }
                Expr::Form(new_form)
            }
            Expr::If { cond, then, else_ } => {
                let new_cond = Box::new(self.convert_helper(*cond, namer));
                let new_then = Box::new(self.convert_helper(*then, namer));
                let new_else = Box::new(self.convert_helper(*else_, namer));
                Expr::If {
                    cond: new_cond,
                    then: new_then,
                    else_: new_else,
                }
            }
            Expr::Let { bindings, body } => {
                assert!(
                    bindings.len() == 1,
                    "Let expression can only have 1 binding. Did you expand it first?"
                );
                let (x, e) = bindings[0].clone();
                let new_e = self.convert_helper(e, namer);
                let new_body = self.convert_helper(*body, namer);
                Expr::Let {
                    bindings: vec![(x, new_e)],
                    body: Box::new(new_body),
                }
            }
            Expr::LetFun {
                name,
                args,
                fun_body,
                expr_body,
            } => {
                let new_fun_body = self.convert_helper(*fun_body, namer);
                let new_expr_body = self.convert_helper(*expr_body, namer);

                // Create bounded set for function body
                // (includes the function name, args and the global labels generated so far)
                let mut fun_bounded = self.freevars_exclude.clone();
                fun_bounded.insert(name.clone());
                for arg in &args {
                    fun_bounded.insert(arg.clone());
                }

                let freevars: Vec<String> = new_fun_body.free_vars(&fun_bounded);
                let closid = namer.next("@f");
                self.freevars_exclude.insert(closid.clone());
                let new_fun_body = Self::id_map(new_fun_body, &name, &closid);
                if freevars.is_empty() {
                    self.global_defs.push(Expr::Defun {
                        name: closid.clone(),
                        args,
                        body: Box::new(new_fun_body),
                    });
                    return Self::id_map(new_expr_body, &name, &closid);
                } else {
                    self.global_defs.push(Expr::DefClos {
                        name: closid.clone(),
                        freevars: freevars.clone(),
                        args,
                        body: Box::new(new_fun_body),
                    });
                    return Expr::LetClos {
                        name,
                        closid,
                        freevars,
                        body: Box::new(new_expr_body),
                    };
                }
            }
            Expr::Def { x, y } => {
                let new_y = self.convert_helper(*y, namer);
                Expr::Def {
                    x,
                    y: Box::new(new_y),
                }
            }
            Expr::Defun { name, args, body } => {
                let new_body = self.convert_helper(*body, namer);
                Expr::Defun {
                    name,
                    args,
                    body: Box::new(new_body),
                }
            }
        }
    }

    fn convert(&mut self, prog: Vec<Expr>, namer: &mut NameGenerator) -> Vec<Expr> {
        let mut new_prog = Vec::new();

        for expr in prog {
            let converted_expr = self.convert_helper(expr, namer);
            new_prog.push(converted_expr);
        }

        [self.global_defs.clone(), new_prog].concat()
    }
}

pub fn closure_convert(prog: Vec<Expr>, namer: &mut NameGenerator) -> Vec<Expr> {
    let mut converter = ClosureConverter::new();
    converter.convert(prog, namer)
}

#[cfg(test)]
mod test {
    use crate::{
        compl::{
            anormal::a_normalize, closure::closure_convert, copyprop::copy_prop,
            knormal::k_normalize, util::NameGenerator,
        },
        format_prog, read_string,
    };

    #[test]
    fn test_closure_convert() {
        let mut namer = NameGenerator::new();
        let prog = read_string(
            r#"
(defun square (x) (* x x))
(let ((f (fn (x) (+ (* x x) x)))) (f (+ 1 (* 2 3))))
(let ((y 1)) (let ((f (fn (x) (+ x y)))) (f (+ 1 (* 2 3)))))
"#,
        )
        .unwrap();
        let kprog = k_normalize(prog.clone(), &mut crate::compl::util::NameGenerator::new());
        let aprog = a_normalize(kprog.clone());
        let cprog = copy_prop(aprog.clone());
        let converted = closure_convert(cprog.clone(), &mut namer);
        println!("original:\n{}", format_prog(&prog));
        println!("k-normalized:\n{}", format_prog(&kprog));
        println!("a-normalized:\n{}", format_prog(&aprog));
        println!("copy-propagated:\n{}", format_prog(&cprog));
        println!("closure-converted:\n{}", format_prog(&converted));
    }
}
