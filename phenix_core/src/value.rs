use std::{borrow::Cow, collections::HashMap, path::PathBuf, rc::Rc};

use rust_decimal::Decimal;

use crate::Creation;

pub use self::{
  action::ActionValue, boolean::BooleanValue, ext::ValueExt, number::NumberValue, path::PathValue,
  string::StringValue,
};

mod action;
mod boolean;
mod ext;
mod number;
mod path;
mod string;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<'a> {
  Boolean(BooleanValue<'a>),
  Number(NumberValue<'a>),
  Path(PathValue<'a>),
  String(StringValue<'a>),
  Action(ActionValue),
}

impl<'a> ValueExt<'a> for Value<'a> {
  fn eval(&self, arguments: Rc<HashMap<Cow<'a, str>, Creation<'a>>>) -> Result<Value, String> {
    match self {
      Self::Boolean(value) => value.eval(arguments),
      Self::Number(value) => value.eval(arguments),
      Self::Path(value) => value.eval(arguments),
      Self::String(value) => value.eval(arguments),
      Self::Action(value) => value.eval(arguments),
    }
  }

  fn to_bool(self) -> Option<bool> {
    match self {
      Self::Boolean(value) => value.to_bool(),
      Self::Number(value) => value.to_bool(),
      Self::Path(value) => value.to_bool(),
      Self::String(value) => value.to_bool(),
      Self::Action(value) => value.to_bool(),
    }
  }

  fn to_int(self) -> Option<i32> {
    match self {
      Self::Boolean(value) => value.to_int(),
      Self::Number(value) => value.to_int(),
      Self::Path(value) => value.to_int(),
      Self::String(value) => value.to_int(),
      Self::Action(value) => value.to_int(),
    }
  }

  fn to_decimal(self) -> Option<Decimal> {
    match self {
      Self::Boolean(value) => value.to_decimal(),
      Self::Number(value) => value.to_decimal(),
      Self::Path(value) => value.to_decimal(),
      Self::String(value) => value.to_decimal(),
      Self::Action(value) => value.to_decimal(),
    }
  }

  fn to_path(self) -> Option<PathBuf> {
    match self {
      Self::Boolean(value) => value.to_path(),
      Self::Number(value) => value.to_path(),
      Self::Path(value) => value.to_path(),
      Self::String(value) => value.to_path(),
      Self::Action(value) => value.to_path(),
    }
  }

  fn to_string(self) -> Option<String> {
    match self {
      Self::Boolean(value) => value.to_string(),
      Self::Number(value) => value.to_string(),
      Self::Path(value) => value.to_string(),
      Self::String(value) => value.to_string(),
      Self::Action(value) => value.to_string(),
    }
  }
}

impl From<bool> for Value {
  fn from(boolean: bool) -> Self {
    Self::Boolean(boolean.into())
  }
}

impl From<i32> for Value {
  fn from(number: i32) -> Self {
    Self::Number(number.into())
  }
}

impl From<PathBuf> for Value {
  fn from(path: PathBuf) -> Self {
    Self::Path(path.into())
  }
}

impl From<&str> for Value {
  fn from(string: &str) -> Self {
    Self::String(string.into())
  }
}

impl From<String> for Value {
  fn from(string: String) -> Self {
    Self::String(string.into())
  }
}

impl From<Vec<String>> for Value {
  fn from(strings: Vec<String>) -> Self {
    Self::String(strings.into())
  }
}

impl From<BooleanValue> for Value {
  fn from(value: BooleanValue) -> Self {
    Self::Boolean(value)
  }
}

impl From<NumberValue> for Value {
  fn from(value: NumberValue) -> Self {
    Self::Number(value)
  }
}

impl From<PathValue> for Value {
  fn from(value: PathValue) -> Self {
    Self::Path(value)
  }
}

impl From<StringValue> for Value {
  fn from(value: StringValue) -> Self {
    Self::String(value)
  }
}

impl From<ActionValue> for Value {
  fn from(value: ActionValue) -> Self {
    Self::Action(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from {
    use super::*;

    #[test]
    fn from_true() {
      let expected = Value::Boolean(true.into());
      let actual = true.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_false() {
      let expected = Value::Boolean(false.into());
      let actual = false.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_i32() {
      let expected = Value::Number(1.into());
      let actual = 1.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_path() {
      let expected = Value::Path(PathBuf::new().into());
      let actual = PathBuf::new().into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_string() {
      let expected = Value::String("test".to_owned().into());
      let actual = "test".to_owned().into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_strings() {
      let expected = Value::String(vec!["test1".to_owned(), "test2".to_owned()].into());
      let actual = vec!["test1".to_owned(), "test2".to_owned()].into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_boolean_value() {
      let expected: Value = true.into();
      let actual = BooleanValue::from(true).into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_number_value() {
      let expected: Value = 1.into();
      let actual = NumberValue::from(1).into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_path_value() {
      let expected: Value = PathBuf::new().into();
      let actual = PathValue::from(PathBuf::new()).into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_string_value() {
      let expected: Value = "test".to_owned().into();
      let actual = StringValue::from("test".to_owned()).into();
      assert_eq!(expected, actual);
    }
  }

  mod value_ext {}
}
