use derive_more::{Display, Error};

use crate::AnyValue;

#[derive(Clone, Debug, Display, Error)]
#[display(fmt = "Can not extract type {to_type} from value {value:?}")]
pub struct ExtractTypeFromAnyError {
  value: AnyValue,
  to_type: ToType,
}

impl ExtractTypeFromAnyError {
  pub fn new<V>(value: V, to_type: ToType) -> Self
  where
    V: Into<AnyValue>,
  {
    Self {
      value: value.into(),
      to_type,
    }
  }
}

#[derive(Clone, Copy, Debug, Display)]
pub enum ToType {
  Action,
  Boolean,
  Command,
  Number,
  Path,
  String,
}
