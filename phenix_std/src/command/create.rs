use phenix_core::{ActionValue, CommandValue, GetArgumentOperation};

pub(crate) fn create_command_value() -> ActionValue {
  let name = GetArgumentOperation::new("std:command$name");
  let arguments = GetArgumentOperation::new("std:command$arguments");
  CommandValue::new(name, arguments).into()
}
