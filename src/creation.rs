use std::{cell::RefCell, rc::Rc};

use rust_lisp::model::{Env, Value};

use super::{
    namespace::{Name, Namespace},
    Qualifier,
};

pub(crate) struct Creation {
    namespace: Namespace,
    arguments: Vec<Argument>,
}

impl Creation {
    pub(crate) fn new(namespace: Namespace, arguments: Vec<Argument>) -> Self {
        Self {
            namespace,
            arguments,
        }
    }

    pub(crate) fn extend_env(&self, env: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let mut env = Env::extend(env);

        for argument in self.arguments.iter() {
            let qualifier = argument.get_qualifier(self.namespace.clone());
            env.define(qualifier.into(), argument.value.clone());
        }

        Rc::new(RefCell::new(env))
    }

    pub(crate) fn get_namespace(&self) -> &Namespace {
        &self.namespace
    }
}

pub(crate) struct Argument {
    name: Name,
    value: Value,
}

impl Argument {
    pub(crate) fn new(name: Name, value: Value) -> Self {
        Self { name, value }
    }

    fn get_qualifier(&self, namespace: Namespace) -> Qualifier {
        Qualifier::new(namespace, self.name.clone())
    }
}
