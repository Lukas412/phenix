pub use self::{
  add::AddOperation, and::AndOperation, condition::ConditionOperation, equals::EqualsOperation,
  get_argument::GetArgumentOperation, join::JoinOperation, or::OrOperation, sub::SubOperation,
  to_boolean::ToBooleanOperation,
};

mod add;
mod and;
mod condition;
mod equals;
mod get_argument;
mod join;
mod or;
mod sub;
mod to_boolean;
