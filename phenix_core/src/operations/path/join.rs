use crate::evaluate::EvaluateResult;
use crate::{ContextExt, Evaluate, PathExpression, PathValue, Runtime};

#[derive(Clone, Debug)]
pub struct PathJoinOperation<Context> {
  expressions: Vec<PathExpression<Context>>,
}

impl<Context> PathJoinOperation<Context> {
  pub fn new(expressions: Vec<PathExpression<Context>>) -> Self {
    Self { expressions }
  }
}

impl<Context> Evaluate<Context> for PathJoinOperation<Context>
where
  Context: ContextExt,
{
  type Result = PathValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    self
      .expressions
      .iter()
      .map(|expression| expression.evaluate(runtime, context))
      .collect()
  }
}
