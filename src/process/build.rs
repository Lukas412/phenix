use rust_lisp::model::Value;

use crate::constructs::{Bundle, Creation, ValueType};

pub(crate) fn build(bundle: &Bundle, creation: &Creation) -> Result<Value, BuildError> {
    Ok(Value::NIL)
}

pub(crate) enum BuildError {
    ValueError {
        expected: ValueType,
        actual: ValueType,
    },
}
