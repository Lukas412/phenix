use std::{borrow::Cow, collections::HashMap, path::PathBuf, rc::Rc};

use rust_decimal::Decimal;

use crate::{Creation, Identifier, Value};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringValue<'a> {
  Value(Cow<'a, str>),
  Join {
    values: Vec<Self>,
    separator: Option<Box<Self>>,
  },
  GetArgument(Identifier<'a>),
}

impl<'a> ValueExt<'a> for StringValue<'a> {
  fn eval(&self, arguments: Rc<HashMap<Cow<'a, str>, Creation<'a>>>) -> Result<Value, String> {
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

  fn to_decimal(self) -> Option<Decimal> {
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
      Self::GetArgument(_) => None,
    }
  }
}

impl<'a> From<&str> for StringValue<'a> {
  fn from(string: &str) -> Self {
    Self::Value(string.into())
  }
}

impl<'a> From<String> for StringValue<'a> {
  fn from(string: String) -> Self {
    Self::Value(string.into())
  }
}

impl<'a> From<Vec<String>> for StringValue<'a> {
  fn from(strings: Vec<String>) -> Self {
    Self::Join {
      values: strings.into_iter().map(Into::into).collect(),
      separator: None,
    }
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
      let expected = StringValue::Join {
        values: vec!["test1".into(), "test2".into()],
        separator: None,
      };
      let actual = vec!["test1".to_owned(), "test2".to_owned()].into();
      assert_eq!(expected, actual);
    }
  }

  mod value_ext {}
}
