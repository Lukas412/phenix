use crate::evaluate::EvaluateResult;
use crate::{ContextExt, Evaluate, Runtime, TextExpression, TextJoinOperation, TextValue};

#[derive(Clone, Debug)]
pub struct TextLinesOperation<Context> {
  operation: TextJoinOperation<TextValue, TextExpression<Context>>,
}

impl<Context> TextLinesOperation<Context> {
  pub fn new(expressions: Vec<TextExpression<Context>>) -> Self {
    Self {
      operation: TextJoinOperation::new("\n", expressions),
    }
  }
}

impl<Context> Evaluate<Context> for TextLinesOperation<Context>
where
  Context: ContextExt,
{
  type Result = TextValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    self.operation.evaluate(runtime, context)
  }
}
