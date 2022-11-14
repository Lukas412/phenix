use crate::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pack {
  value: Value,
}

impl Pack {
  pub fn new(value: Value) -> Self {
    Self { value }
  }
}
