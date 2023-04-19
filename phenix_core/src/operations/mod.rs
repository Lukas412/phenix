pub use self::{
  add::AddOperation,
  and::{And, AndOperation},
  boolean::{HasArgumentOperation, ToBooleanOperation},
  condition::ConditionOperation,
  context::{ContextExtendOperation, ContextSwitchOperation},
  equals::EqualsOperation,
  get_argument::GetArgumentOperation,
  or::{Or, OrOperation},
  path::{PathJoinOperation, ToPathOperation},
  string::{TextBlockOperation, TextJoinOperation, TextLinesOperation, TextWordsOperation},
  sub::SubOperation,
};

mod add;
mod and;
mod boolean;
mod condition;
mod context;
mod equals;
mod get_argument;
mod or;
mod path;
mod string;
mod sub;
