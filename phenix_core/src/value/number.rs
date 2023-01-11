use crate::{BorrowedIdentifier, BorrowedValue, CreationArguments, Runtime};
use duplicate::duplicate_item;
use rust_decimal::Decimal;

use super::{ConcreteValue, ValueExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Number {
  Integer(i32),
  Unsigned(u32),
  Decimal(Decimal),
}

#[duplicate_item(
  value_type_name value_type;
  [Self::Integer] [i32];
  [Self::Unsigned] [u32];
  [Self::Decimal] [Decimal];
)]
impl From<value_type> for Number {
  fn from(value: value_type) -> Self {
    value_type_name(value)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BorrowedNumberValue<'a> {
  Value(Number),
  GetArgument(BorrowedIdentifier<'a>),
}

impl<'a> ValueExt for BorrowedNumberValue<'a> {
  fn eval<'b>(
    &'b self,
    runtime: &'b Runtime,
    arguments: CreationArguments<'b>,
  ) -> Result<BorrowedValue<'static>, String> {
    match self {
      Self::Value(value) => Ok(value.to_owned().into()),
      Self::GetArgument(_) => todo!(),
    }
  }

  fn get_concrete(&self) -> Option<ConcreteValue> {
    match self {
      Self::Value(value) => Some(value.clone().into()),
      _ => None,
    }
  }
}

#[duplicate_item(
  value_type;
  [i32];
  [u32];
  [Decimal];
  [Number];
)]
impl<'a> From<value_type> for BorrowedNumberValue<'a> {
  fn from(number: value_type) -> Self {
    Self::Value(number.into())
  }
}
