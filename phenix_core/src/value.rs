use std::path::PathBuf;

use crate::Pack;

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

impl Value {
  pub fn package(self) -> Pack {
    Pack::new(self)
  }
}

impl ValueExt for Value {
  fn is_concrete(&self) -> bool {
    match self {
      Value::Boolean(value) => value.is_concrete(),
      Value::Number(value) => value.is_concrete(),
      Value::Path(value) => value.is_concrete(),
      Value::String(value) => value.is_concrete(),
      Value::Action(value) => value.is_concrete(),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_package() {
    let expected = Pack::new("test".into());
    let actual = Value::from("test").package();
    assert_eq!(expected, actual);
  }

  mod from {
    use super::*;

    #[test]
    fn from_true() {
      let expected = Value::Boolean(BooleanValue::True);
      let actual = true.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_false() {
      let expected = Value::Boolean(BooleanValue::False);
      let actual = false.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_i32() {
      let expected = Value::Number(NumberValue::Int(1));
      let actual = 1.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_path() {
      let expected = Value::Path(PathValue::Value(PathBuf::new()));
      let actual = PathBuf::new().into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_string() {
      let expected = Value::String(StringValue::Value("test".to_owned()));
      let actual = "test".to_owned().into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_strings() {
      let expected = Value::String(StringValue::List(vec![
        "test1".to_owned(),
        "test2".to_owned(),
      ]));
      let actual = vec!["test1".to_owned(), "test2".to_owned()].into();
      assert_eq!(expected, actual);
    }
  }

  mod value_ext {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn boolean_is_concrete() {
      let value: Value = true.into();
      assert!(value.is_concrete());
    }

    #[test]
    fn number_is_concrete() {
      let value: Value = 1.into();
      assert!(value.is_concrete());
    }

    #[test]
    fn path_is_concrete() {
      let value: Value = PathBuf::new().into();
      assert!(value.is_concrete());
    }

    #[test]
    fn string_is_concrete() {
      let value: Value = "test".to_owned().into();
      assert!(value.is_concrete());
    }
  }
}
