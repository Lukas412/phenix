use crate::evaluate::EvaluateResult;
use crate::{ComplexCreationArguments, Evaluate, JoinOperation, Runtime, TextValue};

#[derive(Clone, Debug)]
pub struct LinesOperation<Expression> {
  operation: JoinOperation<TextValue, Expression>,
}

impl<Expression> LinesOperation<Expression> {
  pub fn new(expressions: Vec<Expression>) -> Self {
    Self {
      operation: JoinOperation::new("\n", expressions),
    }
  }
}

impl<Expression> Evaluate for LinesOperation<Expression>
where
  Expression: Evaluate<Result = TextValue>,
{
  type Result = TextValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    self.operation.evaluate(runtime, arguments)
  }
}
