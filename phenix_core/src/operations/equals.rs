use crate::{BooleanValue};
use std::fmt::Debug;

pub trait EvaluateEquals<Rhs = Self> {
  fn evaluate_equals(self, rhs: Rhs) -> BooleanValue;
}

#[derive(Clone, Debug)]
pub struct EqualsOperation<T> {
  expression: T,
  rhs: T,
}
