mod basics;
mod math;

use crate::intpt::Env;

pub fn load_prelude(env: &mut Env) {
    basics::load_basics(env);
    math::load_math(env);
}
