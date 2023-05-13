use std::fmt::Debug;
use std::ops::Sub;

use crate::evaluate::EvaluateResult;
use crate::{Evaluate, Runtime};

#[derive(Clone, Debug)]
pub struct SubOperation<Expression, OtherExpression = Expression> {
  expressions: Box<(Expression, OtherExpression)>,
}

impl<Expression, OtherExpression> SubOperation<Expression, OtherExpression> {
  pub fn new<IntoExpression, IntoOtherExpression>(
    expression: IntoExpression,
    other: IntoOtherExpression,
  ) -> Self
  where
    IntoExpression: Into<Expression>,
    IntoOtherExpression: Into<OtherExpression>,
  {
    Self {
      expressions: (expression.into(), other.into()).into(),
    }
  }
}

impl<Expression, OtherExpression, Value, Context> Evaluate<Context>
  for SubOperation<Expression, OtherExpression>
where
  Expression: Evaluate<Context>,
  OtherExpression: Evaluate<Context>,

  Expression::Result: Sub<OtherExpression::Result, Output = Value>,
{
  type Result = Value;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Value> {
    self
      .expressions
      .evaluate(runtime, context)
      .map(|(result, other)| result - other)
  }
}
