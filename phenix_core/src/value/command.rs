use crate::{Expression, StringExpression};

use super::array::ArrayExpression;

pub type CommandExpression = Expression<CommandValue, CommandOperation>;

pub struct CommandValue {
  name: StringExpression,
  arguments: ArrayExpression<StringExpression>,
}

pub enum CommandOperation {
  GetArgument(),
}
