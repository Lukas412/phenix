use std::fmt::Debug;

pub trait EvaluateOr<Rhs = Self> {
  type Output;

  fn evaluate_or(self, rhs: Rhs) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct OrOperation<T, Rhs = T>
  where
    T: EvaluateOr<Rhs> + Clone + Debug,
    Rhs: Clone + Debug,
{
  expression: T,
  rhs: Rhs,
}
