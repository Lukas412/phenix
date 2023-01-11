use std::ops::{BitAnd, BitOr};

use rust_decimal::Decimal;

use crate::{BorrowedIdentifier, BorrowedValue, CreationArguments, Runtime};

use super::{ConcreteValue, ValueExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BorrowedBooleanValue<'a> {
  Value(bool),
  Or(Vec<Self>),
  And(Vec<Self>),
  GetArgument(BorrowedIdentifier<'a>),
}

impl<'a> ValueExt for BorrowedBooleanValue<'a> {
  fn eval<'b>(
    &'b self,
    runtime: &'b Runtime,
    arguments: CreationArguments<'b>,
  ) -> Result<BorrowedValue<'static>, String> {
    match self {
      Self::Or(values) => values
        .iter()
        .fold(Ok(false.into()), |result, value| match result {
          Ok(BorrowedValue::Boolean(Self::Value(false))) => value.eval(runtime, arguments.clone()),
          result => result,
        }),
      Self::And(values) => values
        .iter()
        .fold(Ok(true.into()), |result, value| match result {
          Ok(BorrowedValue::Boolean(Self::Value(true))) => value.eval(runtime, arguments.clone()),
          result => result,
        }),
      _ => Ok(self.clone().into()),
    }
  }

  fn get_concrete(&self) -> Option<ConcreteValue> {
    match self {
      Self::Value(value) => Some(value.clone().into()),
      _ => None,
    }
  }
}

impl<'a> From<bool> for BorrowedBooleanValue<'a> {
  fn from(boolean: bool) -> Self {
    Self::Value(boolean)
  }
}
