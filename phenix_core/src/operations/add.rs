use std::{fmt::Debug, ops::Add};

use crate::{evaluate::EvaluateResult, ComplexCreationArguments, Evaluate, Runtime};

#[derive(Clone, Debug)]
pub struct AddOperation<Expression, OtherExpression = Expression> {
  expressions: Box<(Expression, OtherExpression)>,
}

impl<Expression, OtherExpression> AddOperation<Expression, OtherExpression> {
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

impl<Expression, Other, Value> Evaluate for AddOperation<Expression, Other>
where
  Expression: Evaluate,
  Other: Evaluate,
  Expression::Result: Add<Other::Result, Output = EvaluateResult<Value>>,
{
  type Result = Value;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Value> {
    let (result, other_result) = self.expressions.evaluate(runtime, arguments)?;
    result + other_result
  }
}
