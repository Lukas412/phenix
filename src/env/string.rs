use rust_lisp::{
    model::{Env, FloatType, IntType, Value},
    utils::require_typed_arg,
};

const STRING_FROM_INT: &str = "string_from_int";
const STRING_FROM_FLOAT: &str = "string_from_float";

pub(crate) fn define_string_env(env: &mut Env) {
    env.define(
        STRING_FROM_INT.into(),
        Value::NativeFunc(|env, args| {
            let number = require_typed_arg::<IntType>(STRING_FROM_INT, args, 0)?;
            let string = format!("{}", number);
            Ok(Value::String(string))
        }),
    );
    env.define(
        STRING_FROM_FLOAT.into(),
        Value::NativeFunc(|env, args| {
            let number = require_typed_arg::<FloatType>(STRING_FROM_FLOAT, args, 0)?;
            let string = format!("{}", number);
            Ok(Value::String(string))
        }),
    )
}

