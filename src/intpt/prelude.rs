mod basics;
mod list;
mod math;

use crate::ast::Env;

pub fn load_prelude(env: &mut Env) {
    basics::load_basics(env);
    list::load_list(env);
    math::load_math(env);
}
