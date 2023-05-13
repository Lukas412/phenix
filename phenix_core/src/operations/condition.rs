use crate::evaluate::EvaluateResult;
use crate::{BooleanExpression, ContextExt, Evaluate, Runtime};

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

impl<Expression, Value, Context> Evaluate<Context> for ConditionOperation<Expression>
where
  Expression: Evaluate<Context, Result = Value>,
  Context: ContextExt,
{
  type Result = Value;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    if self.condition.evaluate(runtime, context)? {
      self.then.evaluate(runtime, context)
    } else {
      self.other.evaluate(runtime, context)
    }
  }
}
