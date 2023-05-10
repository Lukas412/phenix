use std::fmt::Debug;

use crate::evaluate::EvaluateResult;
use crate::{BooleanExpression, BooleanValue, Evaluate, EvaluateArguments, Runtime};

#[derive(Clone, Debug)]
pub struct AndOperation {
  expressions: Box<(BooleanExpression, BooleanExpression)>,
}

impl AndOperation {
  pub fn new(
    expression: impl Into<BooleanExpression>,
    other_expression: impl Into<BooleanExpression>,
  ) -> Self {
    Self {
      expressions: (expression.into(), other_expression.into()).into(),
    }
  }
}

impl Evaluate for AndOperation {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    self
      .expressions
      .evaluate(runtime, arguments)
      .map(|(result, other)| result && other)
  }
}
