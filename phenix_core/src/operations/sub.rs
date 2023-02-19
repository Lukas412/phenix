use std::fmt::Debug;
use std::ops::Sub;
use crate::{ComplexCreationArguments, Evaluate, Runtime};
use crate::evaluate::EvaluateResult;

pub trait EvaluateSub<Rhs = Self> {
  type Output;

  fn evaluate_sub(self, rhs: Rhs) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct SubOperation<T, R = T> {
  expression: T,
  rhs_expression: R,
}

impl<T, R> SubOperation<T, R> {
  pub fn new<A, B>(expression: A, rhs_expression: B) -> Self
  where
    A: Into<T>,
    B: Into<R>,
  {
    Self {
      expression: expression.into(),
      rhs_expression: rhs_expression.into(),
    }
  }
}

impl<T, V, R> Evaluate<V> for SubOperation<T, R>
where
  T: Evaluate<V>,
  R: Evaluate<V>,
  V: Sub<V, Output = EvaluateResult<V>>,
{
  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<V> {
    let value = self.expression.evaluate(runtime, arguments.clone())?;
    let rhs_value = self.rhs_expression.evaluate(runtime, arguments)?;
    value - rhs_value
  }
}
