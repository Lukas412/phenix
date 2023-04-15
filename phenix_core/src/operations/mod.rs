pub use self::{
  add::AddOperation,
  and::AndOperation,
  condition::ConditionOperation,
  equals::EqualsOperation,
  get_argument::GetArgumentOperation,
  or::OrOperation,
  path::{PathJoinOperation, ToPathOperation},
  string::{TextJoinOperation, TextLinesOperation, TextWordsOperation},
  sub::SubOperation,
  to_boolean::ToBooleanOperation,
};

mod add;
mod and;
mod condition;
mod equals;
mod get_argument;
mod or;
mod path;
mod string;
mod sub;
mod to_boolean;
