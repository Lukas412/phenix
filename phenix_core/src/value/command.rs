use crate::{Expression, GetArgumentOperation, StringExpression};
use duplicate::duplicate_item;

use super::array::ArrayExpression;

pub type CommandExpression = Expression<CommandValue, CommandOperation>;

#[duplicate_item(FromType; [CommandValue];)]
impl From<FromType> for CommandExpression {
  fn from(value: FromType) -> Self {
    Self::Value(value)
  }
}

#[derive(Clone, Debug)]
pub struct CommandValue {
  name: StringExpression,
  arguments: ArrayExpression<StringExpression>,
}

impl CommandValue {
  pub fn new(name: StringExpression, arguments: ArrayExpression<StringExpression>) -> Self {
    Self { name, arguments }
  }
}

#[derive(Clone, Debug)]
pub enum CommandOperation {
  GetArgument(GetArgumentOperation<CommandValue>),
}
