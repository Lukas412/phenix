use crate::evaluate::EvaluateResult;
use crate::{ComplexCreationArguments, Evaluate, JoinOperation, Runtime, TextValue};

#[derive(Clone, Debug)]
pub struct WordsOperation<Expression> {
  operation: JoinOperation<TextValue, Expression>,
}

impl<Expression> WordsOperation<Expression> {
  pub fn new(expressions: Vec<Expression>) -> Self {
    Self {
      operation: JoinOperation::new(" ", expressions),
    }
  }
}

impl<Into1, Into2, Into3, Expression> From<(Into1, Into2, Into3)> for WordsOperation<Expression>
where
  Into1: Into<Expression>,
  Into2: Into<Expression>,
  Into3: Into<Expression>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::new(vec![values.0.into(), values.1.into(), values.2.into()])
  }
}

impl<Expression, IntoExpression> From<Vec<IntoExpression>> for WordsOperation<Expression>
where
  IntoExpression: Into<Expression>,
{
  fn from(expressions: Vec<IntoExpression>) -> Self {
    let expressions = expressions.into_iter().map(Into::into).collect();
    Self::new(expressions)
  }
}

impl<Expression> Evaluate for WordsOperation<Expression>
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