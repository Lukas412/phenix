use crate::{BooleanValue, ComplexCreationArguments, Evaluate, Runtime};

#[derive(Clone, Debug)]
pub struct ToBooleanOperation<Expression> {
  expression: Box<Expression>,
}

impl<Expression> ToBooleanOperation<Expression> {
  pub fn new<IntoExpression>(expression: IntoExpression) -> Self
  where
    IntoExpression: Into<Expression>,
  {
    let expression = Box::from(expression.into());
    Self { expression }
  }
}

impl<Expression> Evaluate for ToBooleanOperation<Expression>
where
  Expression: Evaluate,
  Expression::Result: Into<bool>,
{
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> crate::evaluate::EvaluateResult<Self::Result> {
    self
      .expression
      .evaluate(runtime, arguments)
      .map(|result| result.into())
      .map(|boolean: bool| boolean.into())
  }
}
