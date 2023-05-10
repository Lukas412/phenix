use crate::{BooleanValue, Evaluate, Runtime};

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

impl<Expression, Context> Evaluate<Context> for ToBooleanOperation<Expression>
where
  Expression: Evaluate<Context>,
  Expression::Result: Into<BooleanValue>,
{
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    context: &Context,
  ) -> crate::evaluate::EvaluateResult<Self::Result> {
    self.expression.evaluate(runtime, context).map(Into::into)
  }
}
