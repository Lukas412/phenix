use crate::evaluate::EvaluateResult;
use crate::{Evaluate, EvaluateArguments, Runtime};
use std::fmt::Debug;

pub trait Or<Rhs = Self> {
  type Output;

  fn or(self, other: Rhs) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct OrOperation<Expression, OtherExpression = Expression> {
  expressions: Box<(Expression, OtherExpression)>,
}

impl<Expression, OtherExpression> OrOperation<Expression, OtherExpression> {
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

impl<Expression, OtherExpression, Value> Evaluate for OrOperation<Expression, OtherExpression>
where
  Expression: Evaluate,
  OtherExpression: Evaluate,
  Expression::Result: Or<OtherExpression::Result, Output = EvaluateResult<Value>>,
{
  type Result = Value;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    let (result, other_result) = self.expressions.evaluate(runtime, arguments)?;
    result.or(other_result)
  }
}
