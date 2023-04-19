use crate::evaluate::EvaluateResult;
use crate::{Evaluate, EvaluateArguments, Runtime, TextJoinOperation, TextValue};

#[derive(Clone, Debug)]
pub struct TextLinesOperation<Expression> {
  operation: TextJoinOperation<TextValue, Expression>,
}

impl<Expression> TextLinesOperation<Expression> {
  pub fn new(expressions: Vec<Expression>) -> Self {
    Self {
      operation: TextJoinOperation::new("\n", expressions),
    }
  }
}

impl<Expression> Evaluate for TextLinesOperation<Expression>
where
  Expression: Evaluate<Result = TextValue>,
{
  type Result = TextValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    self.operation.evaluate(runtime, arguments)
  }
}
