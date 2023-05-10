use crate::evaluate::EvaluateResult;
use crate::{BooleanExpression, BooleanValue, Evaluate, EvaluateArguments, Runtime};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct OrOperation {
  expressions: Box<(BooleanExpression, BooleanExpression)>,
}

impl OrOperation {
  pub fn new(
    expression: impl Into<BooleanExpression>,
    other_expression: impl Into<BooleanExpression>,
  ) -> Self {
    Self {
      expressions: (expression.into(), other_expression.into()).into(),
    }
  }
}

impl Evaluate for OrOperation {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    self
      .expressions
      .evaluate(runtime, arguments)
      .map(|(result, other)| result || other)
  }
}
