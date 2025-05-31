use crate::Expr;

pub mod anormal;
pub mod closure;
pub mod codegen;
pub mod copyprop;
pub mod knormal;
pub mod runtime;
pub mod util;

pub fn compl_to_ir(prog: Vec<Expr>) -> String {
    let mut namer = util::NameGenerator::new();
    let prog = knormal::k_normalize(prog, &mut namer);
    let prog = anormal::a_normalize(prog);
    let prog = copyprop::copy_prop(prog);
    let prog = closure::closure_convert(prog, &mut namer);
    crate::format_prog(&prog)
}

/// Generate C code directly from a program file
pub fn compl_to_c(prog: Vec<Expr>) -> String {
    let mut namer = util::NameGenerator::new();
    let prog = knormal::k_normalize(prog, &mut namer);
    let prog = anormal::a_normalize(prog);
    let prog = copyprop::copy_prop(prog);
    let prog = closure::closure_convert(prog, &mut namer);
    codegen::generate_c_code(prog)
}
