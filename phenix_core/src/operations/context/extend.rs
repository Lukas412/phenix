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

impl<Expression, Context> Evaluate<Context> for ContextExtendOperation<Expression>
where
  Expression: Evaluate<DynamicContext>,
  Context: Into<DynamicContext>,
{
  type Result = Expression::Result;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    let mut context = context.clone().into();
    context.reserve(self.context.len());
    for (identifier, creation) in self.context.iter() {
      context.insert(identifier.into(), creation.into())
    }
    self.expression.evaluate(runtime, &context)
  }
}
