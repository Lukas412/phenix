use std::{collections::HashMap, rc::Rc};

use crate::{Creation, Value};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanValue {
  True,
  False,
  Or(Vec<Self>),
  And(Vec<Self>),
}

impl ValueExt for BooleanValue {
  fn eval<'a>(&self, arguments: Rc<HashMap<&'a str, Creation<'a>>>) -> Result<Value, String> {
    match self {
      BooleanValue::Or(values) => {
        values
          .iter()
          .fold(Ok(false.into()), |result, value| match result {
            Ok(Value::Boolean(Self::False)) => value.eval(arguments.clone()),
            result => result,
          })
      }
      BooleanValue::And(values) => {
        values
          .iter()
          .fold(Ok(true.into()), |result, value| match result {
            Ok(Value::Boolean(Self::True)) => value.eval(arguments.clone()),
            result => result,
          })
      }
      _ => Ok(self.clone().into()),
    }
  }

  fn to_bool(self) -> Option<bool> {
    match self {
      Self::True => Some(true),
      Self::False => Some(false),
      Self::Or(values) => values.into_iter(),
    }
  }

  fn to_int(self) -> Option<i32> {
    todo!()
  }

  fn to_float(self) -> Option<f32> {
    todo!()
  }

  fn to_path(self) -> Option<std::path::PathBuf> {
    todo!()
  }

  fn to_string(self) -> Option<String> {
    todo!()
  }
}

impl From<bool> for BooleanValue {
  fn from(boolean: bool) -> Self {
    match boolean {
      true => Self::True,
      false => Self::False,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from {
    use super::*;

    #[test]
    fn from_true() {
      let expected = BooleanValue::True;
      let actual = true.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_false() {
      let expected = BooleanValue::False;
      let actual = false.into();
      assert_eq!(expected, actual);
    }
  }
}
