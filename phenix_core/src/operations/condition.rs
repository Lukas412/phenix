use crate::operations::to_boolean::EvaluateToBoolean;

pub struct ConditionOperation<C, T>
  where C: EvaluateToBoolean
{
  condition: C,
  then: T,
  other: T,
}
