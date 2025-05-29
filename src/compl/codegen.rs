use std::io;
use std::path::Path;

use crate::{Expr, format_prog};

use crate::compl::{
    anormal::a_normalize, closure::closure_convert, copyprop::copy_prop, knormal::k_normalize,
    util::NameGenerator,
};

pub fn codegen(prog: Vec<Expr>) -> String {
    let mut namer = NameGenerator::new();
    let prog = k_normalize(prog, &mut namer);
    let prog = a_normalize(prog);
    let prog = copy_prop(prog);
    let prog = closure_convert(prog, &mut namer);
    format_prog(&prog)
}

pub fn compile<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let prog = crate::read_file(file_path)?;
    Ok(codegen(prog))
}
