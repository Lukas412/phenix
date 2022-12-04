use std::path::PathBuf;

use crate::{CreationArguments, Identifier, Runtime, Value};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathValue<'a> {
  Value(PathBuf),
  GetArgument(Identifier<'a>),
}

impl<'a> ValueExt<'a> for PathValue<'a> {
  fn eval(
    &'a self,
    runtime: &'a Runtime,
    arguments: CreationArguments<'a>,
  ) -> Result<Value<'static>, String> {
    match self {
      Self::Value(value) => Ok(value.clone().into()),
      Self::GetArgument(identifier) => {
        let creation = arguments
          .get(identifier)
          .ok_or_else(|| "could not get arument".to_owned())?;
        runtime.eval(creation)
      }
    }
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
