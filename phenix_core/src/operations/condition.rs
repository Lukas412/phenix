use crate::evaluate::EvaluateResult;
use crate::{BooleanExpression, ComplexCreationArguments, Evaluate, Runtime};

pub struct ConditionOperation<Expression> {
  condition: BooleanExpression,
  then: Expression,
  other: Expression,
}

impl<Expression, Value> Evaluate for ConditionOperation<Expression>
where
  Expression: Evaluate<Result = Value>,
{
  type Result = Value;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    if self.condition.evaluate(runtime, arguments)? {
      self.then.evaluate(runtime, arguments)
    } else {
      self.other.evaluate(runtime, arguments)
    }
  }
}
