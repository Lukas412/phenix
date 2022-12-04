use crate::{CreationArguments, Runtime, Value};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionValue {}

impl<'a> ValueExt<'a> for ActionValue {
  fn eval(
    &'a self,
    _runtime: &'a Runtime,
    _arguments: CreationArguments<'a>,
  ) -> Result<Value<'static>, String> {
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
