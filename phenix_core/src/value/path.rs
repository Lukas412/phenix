use std::{borrow::Cow, collections::HashMap, path::PathBuf, rc::Rc};

use crate::{Creation, Identifier};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathValue<'a> {
  Value(PathBuf),
  GetArgument(Identifier<'a>),
}

impl<'a> ValueExt<'a> for PathValue<'a> {
  fn eval(
    &self,
    arguments: Rc<HashMap<Cow<'a, str>, Creation<'a>>>,
  ) -> Result<crate::Value, String> {
    todo!()
  }

  fn to_bool(self) -> Option<bool> {
    None
  }

  fn to_int(self) -> Option<i32> {
    None
  }

  fn to_decimal(self) -> Option<rust_decimal::Decimal> {
    None
  }

  fn to_path(self) -> Option<PathBuf> {
    match self {
      Self::Value(value) => Some(value),
      Self::GetArgument(_) => None,
    }
  }

  fn to_string(self) -> Option<String> {
    None
  }
}

impl<'a> From<PathBuf> for PathValue<'a> {
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
