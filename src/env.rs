mod string;

use rust_lisp::model::{Env, Value};

use self::string::define_string_env;

pub(crate) fn new_base_env() -> Env {
    let mut env = Env::new();

    define_string_env(&mut env);

    env
}
