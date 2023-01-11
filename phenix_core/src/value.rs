use std::{
  borrow::Cow,
  path::{Path, PathBuf},
};

use duplicate::duplicate_item;
use rust_decimal::Decimal;

use crate::{CreationArguments, Runtime};

use self::dynamic::DynamicValue;
pub use self::{
  action::{Action, BorrowedActionValue},
  boolean::BorrowedBooleanValue,
  concrete::ConcreteValue,
  ext::ValueExt,
  number::{BorrowedNumberValue, Number},
  path::BorrowedPathValue,
  string::BorrowedStringValue,
};

mod action;
mod boolean;
mod concrete;
mod dynamic;
mod ext;
mod number;
mod path;
mod string;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<C, D> {
  Concrete(C),
  Dynamic(D),
}
