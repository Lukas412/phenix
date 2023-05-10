use crate::evaluate::EvaluateResult;
use crate::{DynamicContext, Evaluate, PathValue, Runtime};

#[derive(Clone, Debug)]
pub struct ToPathOperation<Expression> {
  expression: Expression,
}

impl<Expression> ToPathOperation<Expression> {
  pub fn new<IntoExpression>(expression: IntoExpression) -> Self
  where
    IntoExpression: Into<Expression>,
  {
    Self {
      expression: expression.into(),
    }
  }
}

impl<Expression> Evaluate for ToPathOperation<Expression>
where
  Expression: Evaluate,
  Expression::Result: Into<PathValue>,
{
  type Result = PathValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &DynamicContext,
  ) -> EvaluateResult<Self::Result> {
    self.expression.evaluate(runtime, arguments).map(Into::into)
  }
}
