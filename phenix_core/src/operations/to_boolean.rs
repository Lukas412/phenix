use crate::BooleanValue;

pub trait EvaluateToBoolean {
  fn to_boolean(self) -> BooleanValue;
}

pub struct ToBooleanOperation<T>
  where T: EvaluateToBoolean
{
  expression: T,
}