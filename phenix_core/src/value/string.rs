use std::{collections::HashMap, path::PathBuf, rc::Rc};

use crate::{Creation, Value};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringValue {
  Value(String),
  Join {
    values: Vec<Self>,
    separator: Option<Box<Self>>,
  },
}

impl ValueExt for StringValue {
  fn eval<'a>(&self, arguments: Rc<HashMap<&'a str, Creation<'a>>>) -> Result<Value, String> {
    match self {
      Self::Join { values, separator } => Ok(
        values
          .iter()
          .map(|value| {
            value
              .eval(arguments.clone())?
              .to_string()
              .ok_or_else(|| "value could not be converted to a string".to_owned())
          })
          .collect::<Result<String, _>>()?
          .into(),
      ),
      value => Ok(value.clone().into()),
    }
  }

  fn to_bool(self) -> Option<bool> {
    self.to_string().map(|s| !s.is_empty())
  }

  fn to_float(self) -> Option<f32> {
    self.to_string()?.parse().ok()
  }

  fn to_int(self) -> Option<i32> {
    self.to_string()?.parse().ok()
  }

  fn to_path(self) -> Option<PathBuf> {
    self.to_string().map(PathBuf::from)
  }

  fn to_string(self) -> Option<String> {
    match self {
      StringValue::Value(value) => Some(value),
      StringValue::Join { values, separator } => {
        let separator = separator
          .and_then(|value| value.to_string())
          .unwrap_or_default();
        let values: Vec<String> = values.into_iter().filter_map(Self::to_string).collect();
        Some(values.join(&separator))
      }
    }
  }
}

impl From<&str> for StringValue {
  fn from(string: &str) -> Self {
    Self::Value(string.to_owned())
  }
}

impl From<String> for StringValue {
  fn from(string: String) -> Self {
    Self::Value(string)
  }
}

impl From<Vec<String>> for StringValue {
  fn from(strings: Vec<String>) -> Self {
    Self::List(strings)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from {
    use super::*;

    #[test]
    fn from_str() {
      let expected = StringValue::Value("test".to_owned());
      let actual = "test".into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_string() {
      let expected = StringValue::Value("test".to_owned());
      let actual = "test".to_owned().into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_strings() {
      let expected = StringValue::List(vec!["test1".to_owned(), "test2".to_owned()]);
      let actual = vec!["test1".to_owned(), "test2".to_owned()].into();
      assert_eq!(expected, actual);
    }
  }

  mod value_ext {}
}
