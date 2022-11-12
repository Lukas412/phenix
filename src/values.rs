use std::{fmt::Display, str::FromStr};

use error_stack::Context;
use rust_lisp::model::{ForeignValue, ForeignValueRc};

#[derive(Clone, Copy, Debug)]
pub(crate) enum ValueType {
    Boolean,
    Integer,
    Number,
    String,
    Path,
    Action,
}

impl ForeignValue for ValueType {
    fn command(
        &mut self,
        env: std::rc::Rc<std::cell::RefCell<rust_lisp::model::Env>>,
        command: &str,
        args: &[rust_lisp::model::Value],
    ) -> Result<rust_lisp::model::Value, rust_lisp::model::RuntimeError> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct NoValueTypeNamedError {
    name: String,
}

impl NoValueTypeNamedError {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for NoValueTypeNamedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no value type named: {}", self.name)
    }
}

impl Context for NoValueTypeNamedError {}

impl FromStr for ValueType {
    type Err = NoValueTypeNamedError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "path" => Ok(Self::Path),
            "action" => Ok(Self::Action),
            _ => Err(NoValueTypeNamedError::new(string.to_owned())),
        }
    }
}
