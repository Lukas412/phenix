use std::{cell::RefCell, rc::Rc};

use rust_lisp::model::{Env, Value};

use super::namespace::{Name, Namespace};

pub(crate) struct Creation<'a> {
    namespace: Namespace<'a>,
    arguments: Vec<Argument<'a>>,
}

impl<'a> Creation<'a> {
    pub(crate) fn new(namespace: Namespace<'a>, arguments: Vec<Argument<'a>>) -> Self {
        Self {
            namespace,
            arguments,
        }
    }

    pub(crate) fn extend_env(&self, env: Rc<RefCell<Env>>) -> Env {
        let mut env = Env::extend(env);

        env
    }
}

pub(crate) struct Argument<'a> {
    name: Name<'a>,
    value: Value,
}
