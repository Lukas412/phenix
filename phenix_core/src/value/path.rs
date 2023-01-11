use std::{
  borrow::Cow,
  path::{Path, PathBuf},
};

use duplicate::duplicate_item;

use crate::{BorrowedIdentifier, BorrowedValue, CreationArguments, Runtime};

use super::{ConcreteValue, ValueExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BorrowedPathValue<'a> {
  Value(Cow<'a, Path>),
  GetArgument(BorrowedIdentifier<'a>),
}

impl<'a> ValueExt for BorrowedPathValue<'a> {
  fn eval<'b>(
    &'b self,
    runtime: &'b Runtime,
    arguments: CreationArguments<'b>,
  ) -> Result<BorrowedValue<'static>, String> {
    match self {
      Self::Value(value) => Ok(value.to_owned().into()),
      Self::GetArgument(identifier) => {
        let creation = arguments
          .get(identifier)
          .ok_or_else(|| "could not get arument".to_owned())?;
        runtime.eval(creation)
      }
    }
  }

  fn get_concrete(&self) -> Option<ConcreteValue> {
    match self {
      Self::Value(value) => Some(value.clone().into_owned().into()),
      _ => None,
    }
  }
}

#[duplicate_item(
  value_type;
  [&'a Path];
  [PathBuf];
  [Cow<'a, Path>];
)]
impl<'a> From<value_type> for BorrowedPathValue<'a> {
  fn from(value: value_type) -> Self {
    Self::Value(value.into())
  }
}
