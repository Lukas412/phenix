use std::{
    cell::RefCell,
    fs::read_to_string,
    io,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
};

use rust_lisp::{
    interpreter::eval,
    model::{Env, RuntimeError, Value},
    parser::parse,
};
use walkdir::DirEntry;

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

    pub(crate) fn load_file<P: AsRef<Path>>(path: &P) -> Result<Self, PackError> {
        let code = read_to_string(path).map_err(|error| PackError::PackNotFound {
            path: path.as_ref().to_owned(),
            error,
        })?;
        let expressions = parse(&code);
        let type_check = expressions.filter_map(Result::ok).next();
        let type_check = eval(, type_check);
    }

    pub(crate) fn eval(&self, env: Rc<RefCell<Env>>) -> Result<Value, PackError> {
        let result = eval(env, &self.expression).map_err(PackError::from)?;

        match (&self.type_check, &result) {
            (ValueType::Boolean, Value::True) => Ok(result),
            (ValueType::Boolean, Value::False) => Ok(result),
            (ValueType::Integer, Value::Int(_)) => Ok(result),
            (ValueType::Number, Value::Float(_)) => Ok(result),
            (ValueType::String, Value::String(_)) => Ok(result),
            (ValueType::Path, Value::Foreign(_)) => Ok(result), // TODO do complete check
            (ValueType::Action, Value::Foreign(_)) => Ok(result), // TODO do complete check
            _ => Err(PackError::new_value_error(*&self.type_check, result)),
        }
    }
}

#[derive(Debug)]
pub(crate) enum PackError {
    RuntimeError(RuntimeError),
    PackNotFound {
        path: PathBuf,
        error: io::Error,
    },
    ValueError {
        expected_type: ValueType,
        actual_value: Value,
    },
}

impl PackError {
    fn new_value_error(expected_type: ValueType, actual_value: Value) -> Self {
        Self::ValueError {
            expected_type,
            actual_value,
        }
    }
}

impl From<RuntimeError> for PackError {
    fn from(error: RuntimeError) -> Self {
        Self::RuntimeError(error)
    }
}
