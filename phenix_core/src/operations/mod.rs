pub use self::{
  add::AddOperation, and::AndOperation, condition::ConditionOperation, equals::EqualsOperation,
  get_argument::GetArgumentOperation, or::OrOperation, sub::SubOperation,
};

mod add;
mod and;
mod condition;
mod equals;
mod get_argument;
mod or;
mod sub;
mod to_boolean;
