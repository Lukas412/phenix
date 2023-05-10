use std::fmt::Debug;
use std::ops::Sub;

use crate::evaluate::EvaluateResult;
use crate::{DynamicContext, Evaluate, Runtime};

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
  Expression::Result: Sub<OtherExpression::Result, Output = EvaluateResult<Value>>,
{
  type Result = Value;

  fn evaluate(&self, runtime: &Runtime, arguments: &DynamicContext) -> EvaluateResult<Value> {
    let (result, other_result) = self.expressions.evaluate(runtime, arguments)?;
    result - other_result
  }
}
