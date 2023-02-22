use phenix_core::{ActionValue, CommandValue, GetArgumentOperation};

pub(crate) fn create_command_value() -> ActionValue {
  let name = GetArgumentOperation::new("std:command$name").into();
  let arguments = GetArgumentOperation::new("std:command$arguments").into();
  CommandValue::new(name, arguments).into()
}
