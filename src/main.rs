#![allow(unused, dead_code)]

use std::str::FromStr;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rust_lisp::interpreter::eval;
use rust_lisp::model::{Env, Value};
use rust_lisp::parser::parse;
use rust_lisp::{default_env, lisp};

use crate::process::load_packs;
pub(crate) use crate::{
    bundle::Bundle,
    creation::{Argument, Creation},
    namespace::{Name, Namespace, Qualifier},
    pack::Pack,
    values::ValueType,
};

mod bundle;
mod creation;
mod env;
mod namespace;
mod pack;
mod pack_builder;
mod process;
mod values;

fn main() {
    let bundle = load_packs().unwrap();
    let env = Rc::new(RefCell::new(Env::new()));
    let creation = Creation::new(
        "test::pack".into(),
        vec![
            Argument::new("variable".into(), Value::Int(5)),
        ],
    );

    let value = bundle.eval(env.clone(), creation);
    dbg!(value);
}
