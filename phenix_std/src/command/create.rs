use phenix_core::{ActionExpression, CommandOperation, GetArgumentOperation};

pub(crate) fn create_command_value() -> ActionExpression {
  let name = GetArgumentOperation::new("std:command$name");
  let arguments = GetArgumentOperation::new("std:command$arguments");
  CommandOperation::new(name, arguments).into()
}
