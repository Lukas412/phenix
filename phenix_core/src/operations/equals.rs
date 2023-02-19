use crate::{BooleanValue, ComplexCreationArguments, Evaluate, Runtime};
use std::fmt::Debug;
use crate::evaluate::EvaluateResult;

#[derive(Clone, Debug)]
pub struct EqualsOperation<T, R = T> {
  expression: T,
  rhs_expression: R,
}

impl<T, R> EqualsOperation<T, R> {
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

impl<T, R> Evaluate for EqualsOperation<T, R>
where
  T: Evaluate,
  R: Evaluate,
  T::Result: PartialEq<R::Result>
{
  type Result = BooleanValue;

  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<Self::Result> {
    let value = self.expression.evaluate(runtime, arguments.clone())?;
    let rhs_value = self.rhs_expression.evaluate(runtime, arguments)?;
    let result = value == rhs_value;
    Ok(result.into())
  }
}

