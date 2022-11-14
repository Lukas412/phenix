use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringValue {
  Value(String),
  List(Vec<String>),
}

impl ValueExt for StringValue {
  fn is_concrete(&self) -> bool {
    matches!(self, Self::Value(_) | Self::List(_))
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

  mod value_ext {
    use super::*;

    #[test]
    fn is_concrete() {
      let value: StringValue = "test".into();
      assert!(value.is_concrete());
    }
  }
}
