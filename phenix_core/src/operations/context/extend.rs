use crate::evaluate::EvaluateResult;
use crate::{Evaluate, EvaluateArguments, Runtime};

pub struct ContextExtendOperation<Expression> {
  context: EvaluateArguments,
  expression: Expression,
}

impl<Expression> ContextExtendOperation<Expression> {
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
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    let mut arguments = arguments.clone();
    arguments.extend(self.context.clone().into_iter());
    self.expression.evaluate(runtime, &arguments)
  }
}
