use crate::{EvaluateError, Runtime};

pub trait Evaluate<Context> {
  type Result;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result>;
}

pub type EvaluateResult<V> = Result<V, EvaluateError>;

impl<Expression, OtherExpression, Context> Evaluate<Context> for (Expression, OtherExpression)
where
  Expression: Evaluate<Context>,
  OtherExpression: Evaluate<Context>,
{
  type Result = (Expression::Result, OtherExpression::Result);

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    Ok((
      self.0.evaluate(runtime, context)?,
      self.1.evaluate(runtime, context)?,
    ))
  }
}
