use crate::evaluate::EvaluateResult;
use crate::{BooleanExpression, DynamicContext, Evaluate, Runtime};

#[derive(Clone, Debug)]
pub struct ConditionOperation<Expression> {
  condition: BooleanExpression,
  then: Box<Expression>,
  other: Box<Expression>,
}

impl<Expression> ConditionOperation<Expression> {
  pub fn new<IntoBooleanExpression, IntoThenExpression, IntoOtherExpression>(
    condition: IntoBooleanExpression,
    then: IntoThenExpression,
    other: IntoOtherExpression,
  ) -> Self
  where
    IntoBooleanExpression: Into<BooleanExpression>,
    IntoThenExpression: Into<Expression>,
    IntoOtherExpression: Into<Expression>,
  {
    Self {
      condition: condition.into(),
      then: Box::new(then.into()),
      other: Box::new(other.into()),
    }
  }
}

impl<Expression, Value> Evaluate for ConditionOperation<Expression>
where
  Expression: Evaluate<Result = Value>,
{
  type Result = Value;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &DynamicContext,
  ) -> EvaluateResult<Self::Result> {
    if self.condition.evaluate(runtime, arguments)? {
      self.then.evaluate(runtime, arguments)
    } else {
      self.other.evaluate(runtime, arguments)
    }
  }
}
