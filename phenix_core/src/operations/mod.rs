pub use self::{
  add::AddOperation,
  boolean::{AndOperation, HasArgumentOperation, OrOperation, ToBooleanOperation},
  condition::ConditionOperation,
  context::{ContextExtendOperation, ContextSwitchOperation},
  equals::EqualsOperation,
  get_argument::GetArgumentOperation,
  path::{PathJoinOperation, ToPathOperation},
  string::{TextBlockOperation, TextJoinOperation, TextLinesOperation, TextWordsOperation},
  sub::SubOperation,
};

mod add;
mod boolean;
mod condition;
mod context;
mod equals;
mod get_argument;
mod path;
mod string;
mod sub;
