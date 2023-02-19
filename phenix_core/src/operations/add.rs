use std::{fmt::Debug, ops::Add};

use crate::{
  evaluate::EvaluateResult, ComplexCreationArguments, Evaluate, Runtime,
};

#[derive(Clone, Debug)]
pub struct AddOperation<T, R = T> {
  expression: T,
  rhs_expression: R,
}

impl<T, R> AddOperation<T, R> {
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

impl<T, V, R> Evaluate for AddOperation<T, R>
where
  T: Evaluate,
  R: Evaluate,
  T::Result: Add<R::Result, Output = EvaluateResult<V>>,
{
  type Result = V;

  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<V> {
    let value = self.expression.evaluate(runtime, arguments.clone())?;
    let rhs_value = self.rhs_expression.evaluate(runtime, arguments)?;
    value + rhs_value
  }
}
