use crate::evaluate::EvaluateResult;
use crate::{DynamicContext, Evaluate, Runtime};

pub struct ContextExtendOperation<Expression> {
  context: DynamicContext,
  expression: Expression,
}

impl<Expression> ContextExtendOperation<Expression> {
  pub fn new<IntoEvaluateArguments, IntoExpression>(
    context: IntoEvaluateArguments,
    expression: IntoExpression,
  ) -> Self
  where
    IntoEvaluateArguments: Into<DynamicContext>,
    IntoExpression: Into<Expression>,
  {
    Self {
      context: context.into(),
      expression: expression.into(),
    }
  }
}

impl<Expression> Evaluate for ContextExtendOperation<Expression>
where
  Expression: Evaluate,
{
  type Result = Expression::Result;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &DynamicContext,
  ) -> EvaluateResult<Self::Result> {
    let mut arguments = arguments.clone();
    arguments.extend(self.context.clone().into_iter());
    self.expression.evaluate(runtime, &arguments)
  }
}
