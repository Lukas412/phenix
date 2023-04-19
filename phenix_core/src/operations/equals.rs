use std::fmt::Debug;

use crate::evaluate::EvaluateResult;
use crate::{BooleanValue, Evaluate, EvaluateArguments, Runtime};

#[derive(Clone, Debug)]
pub struct EqualsOperation<Expression, Other = Expression> {
  expressions: Box<(Expression, Other)>,
}

impl<Expression, Other> EqualsOperation<Expression, Other> {
  pub fn new<IntoExpression, IntoOther>(expression: IntoExpression, other: IntoOther) -> Self
  where
    IntoExpression: Into<Expression>,
    IntoOther: Into<Other>,
  {
    Self {
      expressions: (expression.into(), other.into()).into(),
    }
  }
}

impl<Expression, Other> Evaluate for EqualsOperation<Expression, Other>
where
  Expression: Evaluate,
  Other: Evaluate,
  Expression::Result: PartialEq<Other::Result>,
{
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    self
      .expressions
      .evaluate(runtime, arguments)
      .map(|(result, other)| result == other)
  }
}
