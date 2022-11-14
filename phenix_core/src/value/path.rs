use std::path::PathBuf;

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathValue {
  Value(PathBuf),
}

impl ValueExt for PathValue {
  fn is_concrete(&self) -> bool {
    matches!(self, Self::Value(_))
  }
}

impl From<PathBuf> for PathValue {
  fn from(path: PathBuf) -> Self {
    Self::Value(path)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from {
    use super::*;

    #[test]
    fn from_path() {
      let expected = PathValue::Value(PathBuf::new());
      let actual = PathBuf::new().into();
      assert_eq!(expected, actual);
    }
  }

  mod value_ext {
    use super::*;

    #[test]
    fn is_concrete() {
      let value: PathValue = PathBuf::new().into();
      assert!(value.is_concrete());
    }
  }
}
