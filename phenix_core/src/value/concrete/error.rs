use duplicate::duplicate_item;
use thiserror::Error;

use super::ConcreteValue;

#[derive(Debug, Error)]
#[error("can not convert {value:?} to type {to_type:?}")]
pub struct ConcreteValueConversionError {
  value: ConcreteValue,
  to_type: ConvertedToType,
}

impl ConcreteValueConversionError {
  pub fn new(value: ConcreteValue, to_type: ConvertedToType) -> Self {
    Self { value, to_type }
  }
}

#[derive(Debug)]
pub enum ConvertedToType {
  Boolean,
  Number,
  Path,
  String,
  Action,
}
