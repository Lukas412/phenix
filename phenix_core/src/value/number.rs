use std::{collections::HashMap, path::PathBuf, rc::Rc};

use crate::{Creation, Identifier};
use rust_decimal::{
  prelude::{FromPrimitive, ToPrimitive},
  Decimal,
};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberValue<'a> {
  Int(i32),
  Decimal(Decimal),
  GetArgument(Identifier<'a>),
}

impl<'a> ValueExt<'a> for NumberValue<'a> {
  fn eval(&self, arguments: Rc<HashMap<&'a str, Creation<'a>>>) -> Result<crate::Value, String> {
    match self {
      _ => Ok(self.clone().into()),
    }
  }

  fn to_bool(self) -> Option<bool> {
    self.to_int().map(|value| value != 0)
  }

  fn to_int(self) -> Option<i32> {
    match self {
      Self::Int(value) => Some(value),
      Self::Decimal(value) => value.to_i32(),
      Self::GetArgument(_) => None,
    }
  }

  fn to_decimal(self) -> Option<Decimal> {
    match self {
      Self::Int(value) => FromPrimitive::from_i32(value),
      Self::Decimal(value) => Some(value),
      Self::GetArgument(_) => None,
    }
  }

  fn to_path(self) -> Option<PathBuf> {
    None
  }

  fn to_string(self) -> Option<String> {
    match self {
      Self::Int(value) => Some(value.to_string()),
      Self::Decimal(value) => Some(value.to_string()),
    }
  }
}

impl From<i32> for NumberValue {
  fn from(number: i32) -> Self {
    Self::Int(number)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from {
    use super::*;

    #[test]
    fn from_i32() {
      let expected = NumberValue::Int(1);
      let actual = 1.into();
      assert_eq!(expected, actual);
    }
  }

  mod value_ext {}
}
