use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::{CreationArguments, Runtime, Value};

pub trait ValueExt<'a>: Into<Value<'a>> + Clone {
  fn eval(
    &'a self,
    runtime: &'a Runtime,
    arguments: CreationArguments<'a>,
  ) -> Result<Value<'static>, String>;

  fn to_bool(self) -> Option<bool>;
  fn to_int(self) -> Option<i32>;
  fn to_decimal(self) -> Option<Decimal>;
  fn to_path(self) -> Option<PathBuf>;
  fn to_string(self) -> Option<String>;
}
