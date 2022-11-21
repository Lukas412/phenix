use std::{collections::HashMap, ops::Not, path::PathBuf, rc::Rc};

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
pub enum Value {
  Boolean(BooleanValue),
  Number(NumberValue),
  Path(PathValue),
  String(StringValue),
  Action(ActionValue),
}

impl ValueExt for Value {
  fn eval<'a>(&self, arguments: Rc<HashMap<&'a str, Creation<'a>>>) -> Result<Value, String> {
    match self {
      Value::Boolean(value) => value.eval(arguments),
      Value::Number(value) => value.eval(arguments),
      Value::Path(value) => value.eval(arguments),
      Value::String(value) => value.eval(arguments),
      Value::Action(value) => value.eval(arguments),
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
