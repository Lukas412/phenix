use crate::{EvaluateArguments, EvaluateError, Runtime};

pub trait Evaluate {
  type Result;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result>;
}

pub type EvaluateResult<V> = Result<V, EvaluateError>;

impl<Expression, OtherExpression> Evaluate for (Expression, OtherExpression)
where
  Expression: Evaluate,
  OtherExpression: Evaluate,
{
  type Result = (Expression::Result, OtherExpression::Result);

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    let (expression, other_expression) = self;
    let result = expression.evaluate(runtime, arguments)?;
    let other_result = other_expression.evaluate(runtime, arguments)?;
    Ok((result, other_result))
  }
}
