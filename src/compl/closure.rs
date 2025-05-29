use super::util::NameGenerator;
use crate::Expr;

pub fn closure_convert_helper(
    expr: Expr,
    namer: &mut NameGenerator,
    global_defs: &mut Vec<Expr>,
) -> Expr {
    match expr {
        Expr::And(_)
        | Expr::Or(_)
        | Expr::Not(_)
        | Expr::LetClos { .. }
        | Expr::DefClos { .. }
        | Expr::Fn { .. } => {
            panic!("Expr not allowed: {}", expr);
        }
        Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) | Expr::Id(_) => {
            expr
        }
        Expr::Form(form) => {
            let mut new_form = Vec::new();
            for item in form {
                new_form.push(closure_convert_helper(item, namer, global_defs));
            }
            Expr::Form(new_form)
        }
        Expr::If { cond, then, else_ } => {
            let new_cond = Box::new(closure_convert_helper(*cond, namer, global_defs));
            let new_then = Box::new(closure_convert_helper(*then, namer, global_defs));
            let new_else = Box::new(closure_convert_helper(*else_, namer, global_defs));
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
            let new_e = closure_convert_helper(e, namer, global_defs);
            let new_body = closure_convert_helper(*body, namer, global_defs);
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
            let new_fun_body = closure_convert_helper(*fun_body, namer, global_defs);
            let new_expr_body = closure_convert_helper(*expr_body, namer, global_defs);

            // Create bounded set for function body (includes function name and args)
            let mut fun_bounded = std::collections::HashSet::new();
            fun_bounded.insert(name.clone());
            for arg in &args {
                fun_bounded.insert(arg.clone());
            }

            let freevars: Vec<String> = new_fun_body.free_vars(&fun_bounded);
            let closid = namer.next("@f");
            global_defs.push(Expr::DefClos {
                name: closid.clone(),
                freevars: freevars.clone(),
                args,
                body: Box::new(new_fun_body),
            });

            // Return a let that binds the function name to the closure
            Expr::LetClos {
                name,
                closid,
                freevars,
                body: Box::new(new_expr_body),
            }
        }
        Expr::Def { x, y } => {
            let new_y = closure_convert_helper(*y, namer, global_defs);
            Expr::Def {
                x,
                y: Box::new(new_y),
            }
        }
        Expr::Defun { name, args, body } => {
            let new_body = closure_convert_helper(*body, namer, global_defs);
            Expr::Defun {
                name,
                args,
                body: Box::new(new_body),
            }
        }
    }
}

pub fn closure_convert(prog: Vec<Expr>, namer: &mut NameGenerator) -> Vec<Expr> {
    let mut global_defs = Vec::new();
    let mut new_prog = Vec::new();

    for expr in prog {
        let converted_expr = closure_convert_helper(expr, namer, &mut global_defs);
        new_prog.push(converted_expr);
    }

    [global_defs, new_prog].concat()
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
