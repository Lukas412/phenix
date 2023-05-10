use std::{fmt::Debug, ops::Add};

use crate::{evaluate::EvaluateResult, Evaluate, Runtime};

#[derive(Clone, Debug)]
pub struct AddOperation<Expression, OtherExpression = Expression> {
  expressions: Box<(Expression, OtherExpression)>,
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
      expressions: (expression.into(), other_expression.into()).into(),
    }
  }
}

impl<Expression, Other, Value, Context> Evaluate<Context> for AddOperation<Expression, Other>
where
  Expression: Evaluate<Context>,
  Other: Evaluate<Context>,
  Expression::Result: Add<Other::Result, Output = EvaluateResult<Value>>,
{
  type Result = Value;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Value> {
    self
      .expressions
      .evaluate(runtime, context)
      .map(|(result, other)| result + other)
  }
}
