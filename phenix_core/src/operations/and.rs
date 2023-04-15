use crate::evaluate::EvaluateResult;
use crate::{ComplexCreationArguments, Evaluate, Runtime};
use std::fmt::Debug;

pub trait And<Rhs = Self> {
  type Output;

  fn and(self, rhs: Rhs) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct AndOperation<Expression, OtherExpression = Expression> {
  expressions: Box<(Expression, OtherExpression)>,
}

impl<Expression, OtherExpression> AndOperation<Expression, OtherExpression> {
  pub fn new<IntoExpression, IntoOtherExpression>(
    expression: IntoExpression,
    other_expression: IntoOtherExpression,
  ) -> Self
  where
    IntoExpression: Into<Expression>,
    IntoOtherExpression: Into<OtherExpression>,
  {
    Self {
      expressions: (expression.into(), other_expression.into()).into(),
    }
  }
}

impl<Expression, Other, Value> Evaluate for AndOperation<Expression, Other>
where
  Expression: Evaluate,
  Other: Evaluate,
  Expression::Result: And<Other::Result, Output = EvaluateResult<Value>>,
{
  type Result = Value;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Value> {
    let (result, other_result) = self.expressions.evaluate(runtime, arguments)?;
    And::and(result, other_result)
  }
}
