use std::fmt::Debug;

pub trait EvaluateSub<Rhs = Self> {
  type Output;

  fn evaluate_sub(self, rhs: Rhs) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct SubOperation<T> {
  expression: T,
  rhs: T,
}
