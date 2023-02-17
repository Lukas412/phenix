pub use self::{
  add::{EvaluateAdd, AddOperation},
  and::{EvaluateAnd, AndOperation},
  condition::ConditionOperation,
  equals::{EvaluateEquals, EqualsOperation},
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
