use crate::evaluate::EvaluateResult;
use crate::{ComplexCreationArguments, Evaluate, Runtime};
use std::fmt::Debug;
use std::ops::Sub;

#[derive(Clone, Debug)]
pub struct SubOperation<Expression, OtherExpression = Expression> {
  expressions: (Expression, OtherExpression),
}

impl<Expression, OtherExpression> SubOperation<Expression, OtherExpression> {
  pub fn new<IntoExpression, IntoOtherExpression>(
    expression: IntoExpression,
    rhs_expression: IntoOtherExpression,
  ) -> Self
  where
    IntoExpression: Into<Expression>,
    IntoOtherExpression: Into<OtherExpression>,
  {
    Self {
      expressions: (expression.into(), rhs_expression.into()),
    }
  }
}

impl<T, V, R> Evaluate for SubOperation<T, R>
where
  T: Evaluate,
  R: Evaluate,
  T::Result: Sub<R::Result, Output = EvaluateResult<V>>,
{
  type Result = V;

  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<V> {
    let (value, other_value) = self.expressions.evaluate(runtime, arguments)?;
    value - other_value
  }
}
