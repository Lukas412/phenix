use std::ops::{BitAnd, BitOr};

use rust_decimal::Decimal;

use crate::{CreationArguments, Identifier, Runtime, Value};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanValue<'a> {
  True,
  False,
  Or(Vec<Self>),
  And(Vec<Self>),
  GetArgument(Identifier<'a>),
}

impl<'a> ValueExt<'a> for BooleanValue<'a> {
  fn eval(
    &'a self,
    runtime: &'a Runtime,
    arguments: CreationArguments<'a>,
  ) -> Result<Value<'static>, String> {
    match self {
      BooleanValue::Or(values) => {
        values
          .iter()
          .fold(Ok(false.into()), |result, value| match result {
            Ok(Value::Boolean(Self::False)) => value.eval(runtime, arguments.clone()),
            result => result,
          })
      }
      BooleanValue::And(values) => {
        values
          .iter()
          .fold(Ok(true.into()), |result, value| match result {
            Ok(Value::Boolean(Self::True)) => value.eval(runtime, arguments.clone()),
            result => result,
          })
      }
      _ => Ok(self.clone().into()),
    }
  }

  fn to_bool(self) -> Option<bool> {
    match self {
      Self::True => Some(true),
      Self::False => Some(false),
      Self::Or(values) => values_to_bool(values, false, BitOr::bitor),
      Self::And(values) => values_to_bool(values, true, BitAnd::bitand),
      Self::GetArgument(_) => todo!(),
    }
  }

  fn to_int(self) -> Option<i32> {
    self.to_bool().map(Into::into)
  }

  fn to_decimal(self) -> Option<Decimal> {
    None
  }

  fn to_path(self) -> Option<std::path::PathBuf> {
    None
  }

  fn to_string(self) -> Option<String> {
    self.to_bool().map(|value| match value {
      true => "true".to_owned(),
      false => "false".to_owned(),
    })
  }
}

impl<'a> From<bool> for BooleanValue<'a> {
  fn from(boolean: bool) -> Self {
    match boolean {
      true => Self::True,
      false => Self::False,
    }
  }
}

fn values_to_bool<'a>(
  values: Vec<BooleanValue<'a>>,
  default: bool,
  combine_func: impl Fn(bool, bool) -> bool,
) -> Option<bool> {
  values
    .into_iter()
    .map(BooleanValue::to_bool)
    .fold(Some(default), |result, value| {
      result.zip(value).map(|v| combine_func(v.0, v.1))
    })
}
