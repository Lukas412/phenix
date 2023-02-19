pub use self::{
  add::AddOperation,
  and::{AndOperation, EvaluateAnd},
  condition::ConditionOperation,
  equals::{EqualsOperation, EvaluateEquals},
  get_argument::GetArgumentOperation,
  or::{EvaluateOr, OrOperation},
  sub::{EvaluateSub, SubOperation},
};

mod add;
mod and;
mod condition;
mod get_argument;
mod or;
mod equals;
mod to_boolean;
mod sub;
