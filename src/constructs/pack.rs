use rust_lisp::model::Value;

use super::{namespace::Namespace, values::ValueType};

pub(crate) struct Pack {
    expression: Value,
    type_check: ValueType,
}

impl Pack {
    pub(crate) fn new(expression: Value, type_check: ValueType) -> Self {
        Self {
            expression,
            type_check,
        }
    }
}
