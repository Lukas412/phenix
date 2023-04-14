pub use self::{
  add::AddOperation, and::AndOperation, condition::ConditionOperation, equals::EqualsOperation,
  get_argument::GetArgumentOperation, join::JoinOperation, lines::LinesOperation, or::OrOperation,
  sub::SubOperation, to_boolean::ToBooleanOperation, words::WordsOperation,
};

mod add;
mod and;
mod condition;
mod equals;
mod get_argument;
mod join;
mod lines;
mod or;
mod sub;
mod to_boolean;
mod words;
