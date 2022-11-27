use std::{collections::HashMap, rc::Rc};

use crate::Creation;

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionValue {}

impl ValueExt for ActionValue {
  fn eval<'a>(
    &self,
    arguments: Rc<HashMap<&'a str, Creation<'a>>>,
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

  fn to_path(self) -> Option<std::path::PathBuf> {
    None
  }

  fn to_string(self) -> Option<String> {
    None
  }
}

#[cfg(test)]
mod tests {}
