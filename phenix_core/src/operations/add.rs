use std::{fmt::Debug, ops::Add};

use crate::{evaluate::EvaluateResult, ComplexCreationArguments, Evaluate, Runtime};

#[derive(Clone, Debug)]
pub struct AddOperation<Expression, OtherExpression = Expression> {
  expressions: (Expression, OtherExpression),
}

impl<Expression, OtherExpression> AddOperation<Expression, OtherExpression> {
  pub fn new<IntoExpression, IntoOtherExpression>(
    expression: IntoExpression,
    other_expression: IntoOtherExpression,
  ) -> Self
  where
    IntoExpression: Into<Expression>,
    IntoOtherExpression: Into<OtherExpression>,
  {
    Self {
      expressions: (expression.into(), other_expression.into()),
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
    let (value, other_value) = self.expressions.evaluate(runtime, arguments)?;
    value + other_value
  }
}
