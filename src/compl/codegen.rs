use crate::Expr;

/// Generates C code from a PureLisp program that has been already processed through
/// k-normalization, a-normalization, copy-propagation, and closure conversion.
pub fn generate_c_code(prog: Vec<Expr>) -> String {
    let mut generator = CCodeGenerator::new();
    generator.gen_prog(prog)
}

#[derive(Debug, Clone)]
struct Env(Vec<(String, String)>);

impl Env {
    fn new() -> Self {
        Env(Vec::new())
    }

    fn push(&mut self, x: String, e: String) {
        self.0.push((x, e));
    }

    fn get(&self, id: &str) -> Option<&String> {
        for (from, to) in self.0.iter().rev() {
            if from == id {
                return Some(to);
            }
        }
        None
    }

    fn pop(&mut self) {
        self.0.pop();
    }
}

struct CCodeGenerator {
    fresh_var_counter: usize,
    env: Env,
    func_decl: Vec<String>,
    clos_decl: Vec<String>,
    global_var_decl: Vec<String>,
    main_prog: Vec<String>,
    func_def: Vec<String>,
    clos_def: Vec<String>,
}

impl CCodeGenerator {
    fn new() -> Self {
        CCodeGenerator {
            fresh_var_counter: 0,
            env: Env::new(),
            func_decl: Vec::new(),
            clos_decl: Vec::new(),
            global_var_decl: Vec::new(),
            main_prog: Vec::new(),
            func_def: Vec::new(),
            clos_def: Vec::new(),
        }
    }

    fn fresh_var(&mut self, prefix: &str) -> String {
        let var_name = format!("{}_{}", prefix, self.fresh_var_counter);
        self.fresh_var_counter += 1;
        var_name
    }

    fn load_builtin_env(&mut self) {
        self.env
            .push("+".to_string(), "global_func_add".to_string());
        self.env
            .push("-".to_string(), "global_func_sub".to_string());
        self.env
            .push("*".to_string(), "global_func_mul".to_string());
        self.env
            .push("/".to_string(), "global_func_div".to_string());
        self.env.push("=".to_string(), "global_func_eq".to_string());
        self.env.push(">".to_string(), "global_func_gt".to_string());
        self.env.push("<".to_string(), "global_func_lt".to_string());
        self.env
            .push("<=".to_string(), "global_func_leq".to_string());
        self.env
            .push(">=".to_string(), "global_func_geq".to_string());
    }

    /// Generate C code from the PureLisp program
    fn gen_prog(&mut self, prog: Vec<Expr>) -> String {
        self.load_builtin_env();
        for expr in prog {
            match expr {
                Expr::Def { x, y } => {
                    self.gen_def(x, *y);
                }
                Expr::Defun { name, args, body } => {
                    self.gen_defun(name, args, *body);
                }
                Expr::DefClos {
                    name,
                    freevars,
                    args,
                    body,
                } => {
                    self.gen_defclos(name, freevars, args, *body);
                }
                _ => {
                    self.gen_main_expr(expr);
                }
            }
        }

        // Combine all parts into a complete C program
        self.assemble_program()
    }

    fn gen_def(&mut self, x: String, y: Expr) {
        let (y_code, y_addr) = self.gen_expr(y);
        let func_addr = self.fresh_var("global_var_init_func");
        self.func_decl.push(format!("PLV {}();", func_addr));
        self.func_def.push(format!(
            "PLV {}(){{\n{}\nreturn {};\n}}",
            func_addr, y_code, y_addr
        ));
        let x_addr = self.fresh_var("global_var");
        self.global_var_decl.push(format!("PLV {};", x_addr));
        self.main_prog
            .push(format!("PLV {} = {}();", x_addr, func_addr));
        self.env.push(x, x_addr);
    }

    fn gen_defun(&mut self, name: String, args: Vec<String>, body: Expr) {
        // println!("generate code for defun: {} {:?} {}", name, args, body);
        let funcptr = self.fresh_var("global_func");
        self.env.push(name.clone(), funcptr.clone());
        for (i, arg) in args.iter().enumerate() {
            self.env.push(arg.clone(), format!("args[{}]", i));
        }
        let (body_code, body_addr) = self.gen_expr(body);
        for _ in args.iter() {
            self.env.pop();
        }
        self.func_decl.push(format!("PLV {}(PLV *args);", funcptr));
        self.func_def.push(format!(
            "PLV {}(PLV *args){{\n{}\nreturn {};\n}}",
            funcptr, body_code, body_addr
        ));
        // println!("function {} defined.", name);
        // println!("{:?}", self.env);
    }

    fn gen_defclos(&mut self, name: String, freevars: Vec<String>, args: Vec<String>, body: Expr) {
        let closptr = self.fresh_var("global_clos");
        self.env.push(name.clone(), closptr.clone());
        for (i, freevar) in freevars.iter().enumerate() {
            self.env.push(freevar.clone(), format!("freevars[{}]", i));
        }
        for (i, arg) in args.iter().enumerate() {
            self.env.push(arg.clone(), format!("args[{}]", i));
        }
        let (body_code, body_addr) = self.gen_expr(body);
        for _ in args.iter() {
            self.env.pop();
        }
        for _ in freevars.iter() {
            self.env.pop();
        }
        self.clos_decl
            .push(format!("PLV {}(PLV *freevars, PLV *args);", closptr));
        self.clos_def.push(format!(
            "PLV {}(PLV *freevars, PLV *args){{\n{}\nreturn {};\n}}",
            closptr, body_code, body_addr
        ));
    }

    fn gen_main_expr(&mut self, expr: Expr) {
        let (code, addr) = self.gen_expr(expr);
        self.main_prog.push(code);
        self.main_prog
            .push(format!("__PLV_print(&{});\nputchar('\\n');\n", addr));
    }

    fn gen_expr(&mut self, expr: Expr) -> (String, String) {
        match expr {
            Expr::Or(_)
            | Expr::And(_)
            | Expr::Not(_)
            | Expr::Fn { .. }
            | Expr::LetFun { .. }
            | Expr::Def { .. }
            | Expr::Defun { .. }
            | Expr::DefClos { .. } => {
                panic!("Invalid expr for codegen: {}", expr);
            }
            Expr::Id(id) => {
                if let Some(mapped) = self.env.get(&id) {
                    let mapped = mapped.clone();
                    if mapped.starts_with("global_func") {
                        let addr = self.fresh_var("f");
                        let code = format!("PLV {} = __new_FUNCPTR({});", addr, mapped);
                        (code, addr)
                    } else {
                        ("".to_string(), mapped.clone())
                    }
                } else {
                    panic!("Undefined identifier: {}", id);
                }
            }
            Expr::Nil => {
                let addr = self.fresh_var("tmp");
                let code = format!("PLV {} = __new_NIL();", addr);
                (code, addr)
            }
            Expr::Int(n) => {
                let addr = self.fresh_var("tmp");
                let code = format!("PLV {} = __new_INT({});", addr, n);
                (code, addr)
            }
            Expr::Float(f) => {
                let addr = self.fresh_var("tmp");
                let code = format!("PLV {} = __new_FLOAT({});", addr, f);
                (code, addr)
            }
            Expr::Str(s) => {
                let addr = self.fresh_var("tmp");
                let code = format!("PLV {} = __new_STR(\"{}\");", addr, s);
                (code, addr)
            }
            Expr::Bool(b) => {
                let addr = self.fresh_var("tmp");
                let code = format!("PLV {} = __new_BOOL({});", addr, if b { 1 } else { 0 });
                (code, addr)
            }
            Expr::Form(form) => {
                let mut code = String::new();
                let mut args = Vec::new();
                for item in form {
                    let (item_code, item_addr) = self.gen_expr(item);
                    if !item_code.is_empty() {
                        code.push_str(&format!("{}\n", item_code));
                    }
                    args.push(item_addr);
                }
                let addr = self.fresh_var("tmp");
                let args_addr = self.fresh_var("args");
                code.push_str(&format!("PLV {}[{}];\n", args_addr, args.len()));
                for (i, arg) in args.iter().enumerate() {
                    code.push_str(&format!("{}[{}] = {};\n", args_addr, i, arg));
                }
                code.push_str(&format!("PLV {} = __PL_funcall({});", addr, args_addr));
                (code, addr)
            }
            Expr::If { cond, then, else_ } => {
                let (cond_code, cond_addr) = self.gen_expr(*cond);
                let (then_code, then_addr) = self.gen_expr(*then);
                let (else_code, else_addr) = self.gen_expr(*else_);
                let addr = self.fresh_var("tmp");
                let mut code = format!("{}\n", cond_code);
                code.push_str(&format!("PLV {};", addr));
                code.push_str(&format!(
                    "if ({}.val.b == 1) {{\n{}\n{} = {};\n}} else {{\n{}\n{}={};\n}}",
                    cond_addr, then_code, addr, then_addr, else_code, addr, else_addr
                ));
                (code, addr)
            }
            Expr::Let { bindings, body } => {
                let mut code = String::new();
                assert!(bindings.len() == 1, "Let can only have 1 binding");
                let (x, e) = bindings[0].clone();
                let (e_code, e_addr) = self.gen_expr(e);
                code.push_str(&format!("{}\n", e_code));
                self.env.push(x, e_addr.clone());
                let (body_code, body_addr) = self.gen_expr(*body);
                self.env.pop();
                code.push_str(&format!("{}\n", body_code));
                code.push_str(&format!("__delete_PLV(&{});", e_addr));
                (code, body_addr)
            }
            Expr::LetClos {
                name,
                closid,
                freevars,
                body,
            } => {
                let mut code = String::new();
                let clos_addr = self.fresh_var("clos");
                let closptr_addr = match self.env.get(&closid) {
                    Some(addr) => addr.clone(),
                    None => {
                        panic!("Undefined identifier: {}", closid);
                    }
                };
                let freevars_addr = self.fresh_var("freevars");
                code.push_str(&format!(
                    "PLV *freevars = malloc(sizeof(PLV) * {});\n",
                    freevars.len()
                ));
                for (i, var) in freevars.iter().enumerate() {
                    if let Some(addr) = self.env.get(var) {
                        code.push_str(&format!("{}[{}] = {};\n", freevars_addr, i, addr));
                    } else {
                        panic!("Undefined identifier: {}", var);
                    }
                }
                code.push_str(&format!(
                    "PLV {} = __new_CLOS({}, {});\n",
                    clos_addr, closptr_addr, freevars_addr
                ));
                self.env.push(name, clos_addr.clone());
                let (body_code, body_addr) = self.gen_expr(*body);
                self.env.pop();
                code.push_str(&format!("{}\n", body_code));
                code.push_str(&format!("__delete_PLV(&{});\nfree(freevars);", clos_addr));
                (code, body_addr)
            }
        }
    }

    /// Assemble the complete C program from all the generated code parts
    fn assemble_program(&self) -> String {
        format!(
            "#include \"runtime.c\"\n\n\
             // Function declarations\n{}\n\n\
             // Closure declarations\n{}\n\n\
             // Global variable declarations\n{}\n\n\
             // Main program\nint main() {{\n{}\nreturn 0;\n}}\n\n\
             // Function definitions\n{}\n\n\
             // Closure definitions\n{}",
            self.func_decl.join("\n"),
            self.clos_decl.join("\n"),
            self.global_var_decl.join("\n"),
            self.main_prog.join("\n"),
            self.func_def.join("\n"),
            self.clos_def.join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compl::{
        anormal::a_normalize, closure::closure_convert, copyprop::copy_prop, knormal::k_normalize,
        util::NameGenerator,
    };
    use crate::read_string;

    fn compile_to_c(source: &str) -> String {
        let mut namer = NameGenerator::new();
        let prog = read_string(source).unwrap();
        let kprog = k_normalize(prog.clone(), &mut NameGenerator::new());
        let aprog = a_normalize(kprog.clone());
        let cprog = copy_prop(aprog.clone());
        let converted = closure_convert(cprog.clone(), &mut namer);
        println!("Converted program:\n{}", crate::format_prog(&converted));
        generate_c_code(converted)
    }

    #[test]
    fn test_simple_function() {
        let c_code = compile_to_c(
            r#"
(+ 1 (* 2 3))
"#,
        );
        println!("Simple function test:\n{}", c_code);
    }
}
