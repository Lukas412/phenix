use crate::evaluate::EvaluateResult;
use crate::{DynamicContext, Evaluate, Runtime};

#[derive(Clone, Debug)]
pub struct ContextSwitchOperation<Expression, Context> {
  context: Context,
  expression: Box<Expression>,
}

impl<Expression, Context> ContextSwitchOperation<Expression, Context> {
  pub fn new<IntoEvaluateArguments, IntoExpression>(
    context: IntoEvaluateArguments,
    expression: IntoExpression,
  ) -> Self
  where
    IntoEvaluateArguments: Into<Context>,
    IntoExpression: Into<Expression>,
  {
    Self {
      context: context.into(),
      expression: Box::new(expression.into()),
    }
  }
}

impl<Expression, PreviousContext, Context> Evaluate<PreviousContext>
  for ContextSwitchOperation<Expression, Context>
where
  Expression: Evaluate<Context>,
{
  type Result = Expression::Result;

  fn evaluate(&self, runtime: &Runtime, _context: &Context) -> EvaluateResult<Self::Result> {
    self.expression.evaluate(runtime, &self.context)
  }
}
