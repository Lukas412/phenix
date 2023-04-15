use crate::evaluate::EvaluateResult;
use crate::{ComplexCreationArguments, Evaluate, PathExpression, PathValue, Runtime};

#[derive(Clone, Debug)]
pub struct PathJoinOperation {
  expressions: Vec<PathExpression>,
}

impl PathJoinOperation {
  pub fn new(expressions: Vec<PathExpression>) -> Self {
    Self { expressions }
  }
}

impl Evaluate for PathJoinOperation {
  type Result = PathValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    self
      .expressions
      .iter()
      .map(|expression| expression.evaluate(runtime, arguments))
      .collect()
  }
}
