use crate::evaluate::EvaluateResult;
use crate::{Evaluate, EvaluateArguments, Runtime};

#[derive(Clone, Debug)]
pub struct ContextSwitchOperation<Expression> {
  context: EvaluateArguments,
  expression: Box<Expression>,
}

impl<Expression> ContextSwitchOperation<Expression> {
  pub fn new<IntoEvaluateArguments, IntoExpression>(
    context: IntoEvaluateArguments,
    expression: IntoExpression,
  ) -> Self
  where
    IntoEvaluateArguments: Into<EvaluateArguments>,
    IntoExpression: Into<Expression>,
  {
    Self {
      context: context.into(),
      expression: Box::new(expression.into()),
    }
  }
}

impl<Expression> Evaluate for ContextSwitchOperation<Expression>
where
  Expression: Evaluate,
{
  type Result = Expression::Result;

  fn evaluate(
    &self,
    runtime: &Runtime,
    _arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    self.expression.evaluate(runtime, &self.context)
  }
}
