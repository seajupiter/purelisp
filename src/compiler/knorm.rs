use crate::ast::Expr;

/// Represents a K-normalized expression
#[derive(Debug, Clone, PartialEq)]
pub enum KExpr {
    /// Simple values: nil, booleans, integers, floats, strings
    Atom(Expr),
    /// Variable reference
    Var(String),
    /// Function application
    App { fun: Box<KExpr>, args: Vec<KExpr> },
    /// Let binding
    Let {
        var: String,
        bound_expr: Box<KExpr>,
        body: Box<KExpr>,
    },
    /// If expression
    If {
        cond: Box<KExpr>,
        then_expr: Box<KExpr>,
        else_expr: Box<KExpr>,
    },
    /// Let-scoped function
    LetFun {
        name: String,
        params: Vec<String>,
        fun_body: Box<KExpr>,
        expr_body: Box<KExpr>,
    },
}

/// Fresh variable counter for generating unique variable names
struct FreshVarGen {
    counter: usize,
}

impl FreshVarGen {
    fn new() -> Self {
        FreshVarGen { counter: 0 }
    }

    fn fresh(&mut self, base: &str) -> String {
        let var = format!("{}_{}", base, self.counter);
        self.counter += 1;
        var
    }
}

/// K-normalize an expression
pub fn k_normalize(expr: Expr) -> KExpr {
    let mut var_gen = FreshVarGen::new();
    k_normalize_expr(expr, &mut var_gen)
}

/// Helper function to perform the actual K-normalization
fn k_normalize_expr(expr: Expr, var_gen: &mut FreshVarGen) -> KExpr {
    match expr {
        // Atomic expressions (don't need to be normalized further)
        Expr::Nil | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) => {
            KExpr::Atom(expr)
        }

        // Variable reference
        Expr::Id(id) => KExpr::Var(id),

        // Forms (function applications)
        Expr::Form(exprs) => {
            if exprs.is_empty() {
                panic!("Empty form in k-normalization");
            }

            // Normalize function expression
            let fun_expr = k_normalize_expr(exprs[0].clone(), var_gen);

            // Normalize arguments
            let mut k_args = Vec::new();
            let mut lets = Vec::new();

            for arg in &exprs[1..] {
                let k_arg = k_normalize_expr(arg.clone(), var_gen);

                // If the argument is complex, bind it to a fresh variable
                match &k_arg {
                    KExpr::Atom(_) | KExpr::Var(_) => {
                        k_args.push(k_arg);
                    }
                    _ => {
                        let fresh_var = var_gen.fresh("arg");
                        lets.push((fresh_var.clone(), k_arg));
                        k_args.push(KExpr::Var(fresh_var));
                    }
                }
            }

            // Build the function application
            let mut result = KExpr::App {
                fun: Box::new(fun_expr),
                args: k_args,
            };

            // Wrap with let bindings for complex arguments (in reverse order)
            for (var, bound_expr) in lets.into_iter().rev() {
                result = KExpr::Let {
                    var,
                    bound_expr: Box::new(bound_expr),
                    body: Box::new(result),
                };
            }

            result
        }

        // Let bindings
        Expr::Let { bindings, body } => {
            let mut result = k_normalize_expr(*body, var_gen);

            // Process bindings in reverse order
            for (var, bound_expr) in bindings.into_iter().rev() {
                let k_bound = k_normalize_expr(bound_expr, var_gen);
                result = KExpr::Let {
                    var,
                    bound_expr: Box::new(k_bound),
                    body: Box::new(result),
                };
            }

            result
        }

        // If expressions
        Expr::If { cond, then, else_ } => {
            let k_cond = k_normalize_expr(*cond, var_gen);
            let k_then = k_normalize_expr(*then, var_gen);
            let k_else = k_normalize_expr(*else_, var_gen);

            // If condition is complex, bind it to a fresh variable
            match k_cond {
                KExpr::Atom(_) | KExpr::Var(_) => KExpr::If {
                    cond: Box::new(k_cond),
                    then_expr: Box::new(k_then),
                    else_expr: Box::new(k_else),
                },
                _ => {
                    let cond_var = var_gen.fresh("cond");
                    KExpr::Let {
                        var: cond_var.clone(),
                        bound_expr: Box::new(k_cond),
                        body: Box::new(KExpr::If {
                            cond: Box::new(KExpr::Var(cond_var)),
                            then_expr: Box::new(k_then),
                            else_expr: Box::new(k_else),
                        }),
                    }
                }
            }
        }

        // Function definitions
        Expr::Fn { args, body } => {
            let k_body = k_normalize_expr(*body, var_gen);
            // Create a fresh function name
            let fun_name = var_gen.fresh("fn");
            // The expression body is just the function name
            let expr_body = KExpr::Var(fun_name.clone());

            KExpr::LetFun {
                name: fun_name,
                params: args,
                fun_body: Box::new(k_body),
                expr_body: Box::new(expr_body),
            }
        }

        // And expressions - transform to if expressions first, then k-normalize
        Expr::And(exprs) => {
            if exprs.is_empty() {
                return KExpr::Atom(Expr::Bool(true));
            }

            // Convert (and a b c) to (if a (if b c false) false)
            let mut result = exprs.last().unwrap().clone();

            for expr in exprs.iter().rev().skip(1) {
                result = Expr::If {
                    cond: Box::new(expr.clone()),
                    then: Box::new(result),
                    else_: Box::new(Expr::Bool(false)),
                };
            }

            k_normalize_expr(result, var_gen)
        }

        // Or expressions - transform to if expressions first, then k-normalize
        Expr::Or(exprs) => {
            if exprs.is_empty() {
                return KExpr::Atom(Expr::Bool(false));
            }

            // Convert (or a b c) to (if a true (if b true c))
            let mut result = exprs.last().unwrap().clone();

            for expr in exprs.iter().rev().skip(1) {
                result = Expr::If {
                    cond: Box::new(expr.clone()),
                    then: Box::new(Expr::Bool(true)),
                    else_: Box::new(result),
                };
            }

            k_normalize_expr(result, var_gen)
        }

        // Def expressions
        Expr::Def { x, y } => {
            // Transform def into let
            let k_bound = k_normalize_expr(*y, var_gen);
            KExpr::Let {
                var: x,
                bound_expr: Box::new(k_bound),
                body: Box::new(KExpr::Atom(Expr::Nil)), // Def returns nil
            }
        }

        // Defun expressions
        Expr::Defun { name, args, body } => {
            // Transform defun into (def name (fn args body))
            let fn_expr = Expr::Fn {
                args: args.clone(),
                body: body.clone(),
            };

            let def_expr = Expr::Def {
                x: name,
                y: Box::new(fn_expr),
            };

            k_normalize_expr(def_expr, var_gen)
        }

        // LetFun expressions
        Expr::LetFun {
            name,
            args,
            fun_body,
            expr_body,
        } => {
            KExpr::LetFun {
                name,
                params: args,
                fun_body: Box::new(k_normalize_expr(*fun_body, var_gen)),
                expr_body: Box::new(k_normalize_expr(*expr_body, var_gen)),
            }
        }
    }
}

/// Convert a K-normalized expression back to regular expression
pub fn knorm_to_expr(k_expr: KExpr) -> Expr {
    match k_expr {
        KExpr::Atom(expr) => expr,
        KExpr::Var(id) => Expr::Id(id),
        KExpr::App { fun, args } => {
            let mut form = vec![knorm_to_expr(*fun)];
            for arg in args {
                form.push(knorm_to_expr(arg));
            }
            Expr::Form(form)
        }
        KExpr::Let {
            var,
            bound_expr,
            body,
        } => Expr::Let {
            bindings: vec![(var, knorm_to_expr(*bound_expr))],
            body: Box::new(knorm_to_expr(*body)),
        },
        KExpr::If {
            cond,
            then_expr,
            else_expr,
        } => Expr::If {
            cond: Box::new(knorm_to_expr(*cond)),
            then: Box::new(knorm_to_expr(*then_expr)),
            else_: Box::new(knorm_to_expr(*else_expr)),
        },
        KExpr::LetFun {
            name,
            params,
            fun_body,
            expr_body,
        } => Expr::LetFun {
            name,
            args: params,
            fun_body: Box::new(knorm_to_expr(*fun_body)),
            expr_body: Box::new(knorm_to_expr(*expr_body)),
        },
    }
}
