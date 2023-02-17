use std::fmt::Debug;

pub trait EvaluateAnd<Rhs = Self> {
  type Output;

  fn evaluate_and(self, rhs: Rhs) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct AndOperation<T, Rhs = T>
  where
    T: EvaluateAnd<Rhs> + Clone + Debug,
    Rhs: Clone + Debug,
{
  expression: T,
  rhs: Rhs,
}
