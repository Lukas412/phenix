use std::{borrow::Cow, path::PathBuf};

use rust_decimal::Decimal;

use crate::{CreationArguments, Identifier, Runtime, Value};

use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringValue<'a> {
  Value(Cow<'a, str>),
  Join {
    values: Vec<Self>,
    separator: Option<Box<Self>>,
  },
  GetArgument(Identifier<'a>),
}

impl<'a> ValueExt<'a> for StringValue<'a> {
  fn eval(
    &'a self,
    runtime: &'a Runtime,
    arguments: CreationArguments<'a>,
  ) -> Result<Value<'static>, String> {
    match self {
      Self::Join { values, separator } => Ok(
        values
          .iter()
          .map(|value| {
            value
              .eval(runtime, arguments.clone())?
              .to_string()
              .ok_or_else(|| "value could not be converted to a string".to_owned())
          })
          .collect::<Result<String, _>>()?
          .into(),
      ),
      value => Ok(value.clone().into()),
    }
  }

  fn to_bool(self) -> Option<bool> {
    self.to_string().map(|s| !s.is_empty())
  }

  fn to_decimal(self) -> Option<Decimal> {
    self.to_string()?.parse().ok()
  }

  fn to_int(self) -> Option<i32> {
    self.to_string()?.parse().ok()
  }

  fn to_path(self) -> Option<PathBuf> {
    self.to_string().map(PathBuf::from)
  }

  fn to_string(self) -> Option<String> {
    match self {
      StringValue::Value(value) => Some(value.into()),
      StringValue::Join { values, separator } => {
        let separator = separator
          .and_then(|value| value.to_string())
          .unwrap_or_default();
        let values: Vec<String> = values.into_iter().filter_map(Self::to_string).collect();
        Some(values.join(&separator))
      }
      Self::GetArgument(_) => None,
    }
  }
}

impl<'a> From<&'a str> for StringValue<'a> {
  fn from(string: &'a str) -> Self {
    Self::Value(string.into())
  }
}

impl<'a> From<String> for StringValue<'a> {
  fn from(string: String) -> Self {
    Self::Value(string.into())
  }
}

impl<'a> From<Vec<String>> for StringValue<'a> {
  fn from(strings: Vec<String>) -> Self {
    Self::Join {
      values: strings.into_iter().map(Into::into).collect(),
      separator: None,
    }
  }
}
