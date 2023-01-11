use duplicate::duplicate_item;

use crate::{BorrowedIdentifier, BorrowedValue, CreationArguments, Runtime};

use super::{ConcreteValue, ValueExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BorrowedActionValue<'a> {
  Value(Action),
  GetArgument(BorrowedIdentifier<'a>),
}

impl<'a> ValueExt for BorrowedActionValue<'a> {
  fn eval<'b>(
    &'b self,
    runtime: &'b Runtime,
    arguments: CreationArguments<'b>,
  ) -> Result<BorrowedValue<'static>, String> {
    match self {
      Self::Value(value) => Ok(value.clone().into()),
      Self::GetArgument(identifier) => {
        let creation = arguments
          .get(identifier)
          .ok_or_else(|| "could not get argument".to_owned())?;
        runtime.eval(creation)
      }
    }
  }

  fn get_concrete(&self) -> Option<ConcreteValue> {
    match self {
      Self::Value(value) => Some(value.to_owned()),
      Self::GetArgument(_) => None,
    }
  }
}

#[duplicate_item(
  value_type;
  [Action];
)]
impl<'a> From<value_type> for BorrowedActionValue<'a> {
  fn from(value: value_type) -> Self {
    Self::Value(value.into())
  }
}
