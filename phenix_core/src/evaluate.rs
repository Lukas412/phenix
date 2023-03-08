use crate::{ComplexCreationArguments, EvaluateError, Runtime};

pub trait Evaluate {
  type Result;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
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
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    let (expression, other_expression) = self;
    let value = expression.evaluate(runtime, arguments.clone())?;
    let other_value = other_expression.evaluate(runtime, arguments)?;
    Ok((value, other_value))
  }
}
