#![allow(unused, dead_code)]
mod constructs;
mod env;
mod process;

use rust_lisp::interpreter::eval;
use rust_lisp::model::Value;
use rust_lisp::{default_env, lisp};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::constructs::{Pack, ValueType};
use crate::env::new_base_env;

fn main() {
    let env = Rc::new(RefCell::new(new_base_env()));

    let first_expression = lisp!((string_from_int 4));

    let evaluation_result = eval(env.clone(), &first_expression).unwrap();

    println!("{}", evaluation_result);
}
