use std::borrow::Cow;

use duplicate::duplicate_item;

use crate::{BorrowedIdentifier, BorrowedValue, CreationArguments, Runtime};

use super::{ConcreteValue, ValueExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BorrowedStringValue<'a> {
  Value(Cow<'a, str>),
  Join {
    values: Vec<Self>,
    separator: Option<Box<Self>>,
  },
  GetArgument(BorrowedIdentifier<'a>),
}

impl<'a> ValueExt for BorrowedStringValue<'a> {
  fn eval<'b>(
    &'b self,
    runtime: &'b Runtime,
    arguments: CreationArguments<'b>,
  ) -> Result<BorrowedValue<'static>, String> {
    match self {
      Self::Join { values, separator } => Ok(
        values
          .iter()
          .map(|value| {
            value
              .eval(runtime, arguments.clone())?
              .get_concrete()
              .ok_or_else(|| "value can not be determened".to_owned())
          })
          .map(|value| match value? {
            ConcreteValue::String(value) => Ok(value.to_owned()),
            _ => Err("value could not be converted to a string".to_owned()),
          })
          .collect::<Result<String, _>>()?
          .into(),
      ),
      value => Ok(value.clone().into()),
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
  [&'a str];
  [String];
  [Cow<'a, str>];
)]
impl<'a> From<value_type> for BorrowedStringValue<'a> {
  fn from(string: value_type) -> Self {
    Self::Value(string.into())
  }
}

impl<'a> From<Vec<String>> for BorrowedStringValue<'a> {
  fn from(strings: Vec<String>) -> Self {
    Self::Join {
      values: strings.into_iter().map(Into::into).collect(),
      separator: None,
    }
  }
}
