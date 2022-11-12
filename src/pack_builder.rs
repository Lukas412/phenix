use std::{cell::RefCell, rc::Rc, str::FromStr};

use rust_lisp::{
    model::{Env, RuntimeError, Symbol, Value},
    utils::require_typed_arg,
};

use crate::values::ValueType;

const TYPE_FUNC: &str = "type";

pub(crate) struct PackBuilder {
    env: Rc<RefCell<Env>>,
}

impl PackBuilder {
    pub(crate) fn new() -> Self {
        let env = new_env();
        Self { env }
    }
}

fn new_env() -> Rc<RefCell<Env>> {
    let mut env = Env::new();

    env.define(
        TYPE_FUNC.into(),
        Value::NativeFunc(|env: Rc<RefCell<Env>>, args: &[Value]| {
            let value_type = require_typed_arg::<&String>(TYPE_FUNC, args, 0)?;

            Ok(Value::Foreign(ValueType::from_str(value_type.as_str())))
        }),
    );

    Rc::new(RefCell::new(env))
}

fn native_type_func(env: Rc<RefCell<Env>>, args: &[Value]) -> Result<Value, RuntimeError> {
    let value_type = require_typed_arg::<&String>(TYPE_FUNC, args, 0)?;

    Ok(Value::Foreign(
        ValueType::from_str(value_type.as_str()).into(),
    ))
}
