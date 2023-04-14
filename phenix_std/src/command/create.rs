use phenix_core::{ActionExpression, CommandExpression, GetArgumentOperation};

pub(crate) fn create_command_value() -> ActionExpression {
  let name = GetArgumentOperation::new("std:command$name");
  let arguments = GetArgumentOperation::new("std:command$arguments");
  CommandExpression::new(name, arguments).into()
}
