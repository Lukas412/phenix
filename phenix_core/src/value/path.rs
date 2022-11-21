use std::{collections::HashMap, path::PathBuf, rc::Rc};

use crate::Creation;

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathValue {
  Value(PathBuf),
}

impl ValueExt for PathValue {
  fn eval<'a>(
    &self,
    arguments: Rc<HashMap<&'a str, Creation<'a>>>,
  ) -> Result<crate::Value, String> {
    todo!()
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

  mod value_ext {}
}
