use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::{CreationArguments, Runtime};

pub use self::{
  action::ActionValue, boolean::BooleanValue, ext::ValueExt, number::NumberValue, path::PathValue,
  string::StringValue,
};

mod action;
mod boolean;
mod ext;
mod number;
mod path;
mod string;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<'a> {
  Boolean(BooleanValue<'a>),
  Number(NumberValue<'a>),
  Path(PathValue<'a>),
  String(StringValue<'a>),
  Action(ActionValue),
}

impl<'a> ValueExt<'a> for Value<'a> {
  fn eval(
    &'a self,
    runtime: &'a Runtime,
    arguments: CreationArguments<'a>,
  ) -> Result<Value<'static>, String> {
    match self {
      Self::Boolean(value) => value.eval(runtime, arguments),
      Self::Number(value) => value.eval(runtime, arguments),
      Self::Path(value) => value.eval(runtime, arguments),
      Self::String(value) => value.eval(runtime, arguments),
      Self::Action(value) => value.eval(runtime, arguments),
    }
  }

  fn to_bool(self) -> Option<bool> {
    match self {
      Self::Boolean(value) => value.to_bool(),
      Self::Number(value) => value.to_bool(),
      Self::Path(value) => value.to_bool(),
      Self::String(value) => value.to_bool(),
      Self::Action(value) => value.to_bool(),
    }
  }

  fn to_int(self) -> Option<i32> {
    match self {
      Self::Boolean(value) => value.to_int(),
      Self::Number(value) => value.to_int(),
      Self::Path(value) => value.to_int(),
      Self::String(value) => value.to_int(),
      Self::Action(value) => value.to_int(),
    }
  }

  fn to_decimal(self) -> Option<Decimal> {
    match self {
      Self::Boolean(value) => value.to_decimal(),
      Self::Number(value) => value.to_decimal(),
      Self::Path(value) => value.to_decimal(),
      Self::String(value) => value.to_decimal(),
      Self::Action(value) => value.to_decimal(),
    }
  }

  fn to_path(self) -> Option<PathBuf> {
    match self {
      Self::Boolean(value) => value.to_path(),
      Self::Number(value) => value.to_path(),
      Self::Path(value) => value.to_path(),
      Self::String(value) => value.to_path(),
      Self::Action(value) => value.to_path(),
    }
  }

  fn to_string(self) -> Option<String> {
    match self {
      Self::Boolean(value) => value.to_string(),
      Self::Number(value) => value.to_string(),
      Self::Path(value) => value.to_string(),
      Self::String(value) => value.to_string(),
      Self::Action(value) => value.to_string(),
    }
  }
}

impl<'a> From<bool> for Value<'a> {
  fn from(boolean: bool) -> Self {
    Self::Boolean(boolean.into())
  }
}

impl<'a> From<i32> for Value<'a> {
  fn from(number: i32) -> Self {
    Self::Number(number.into())
  }
}

impl<'a> From<PathBuf> for Value<'a> {
  fn from(path: PathBuf) -> Self {
    Self::Path(path.into())
  }
}

impl<'a> From<&'a str> for Value<'a> {
  fn from(string: &'a str) -> Self {
    Self::String(string.into())
  }
}

impl<'a> From<String> for Value<'a> {
  fn from(string: String) -> Self {
    Self::String(string.into())
  }
}

impl<'a> From<Vec<String>> for Value<'a> {
  fn from(strings: Vec<String>) -> Self {
    Self::String(strings.into())
  }
}

impl<'a> From<BooleanValue<'a>> for Value<'a> {
  fn from(value: BooleanValue<'a>) -> Self {
    Self::Boolean(value)
  }
}

impl<'a> From<NumberValue<'a>> for Value<'a> {
  fn from(value: NumberValue<'a>) -> Self {
    Self::Number(value)
  }
}

impl<'a> From<PathValue<'a>> for Value<'a> {
  fn from(value: PathValue<'a>) -> Self {
    Self::Path(value)
  }
}

impl<'a> From<StringValue<'a>> for Value<'a> {
  fn from(value: StringValue<'a>) -> Self {
    Self::String(value)
  }
}

impl<'a> From<ActionValue> for Value<'a> {
  fn from(value: ActionValue) -> Self {
    Self::Action(value)
  }
}
