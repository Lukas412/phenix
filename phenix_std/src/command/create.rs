use phenix_core::{ActionValue, ArrayValue, GetArgumentOperation, StringOperation};

pub fn create_command_value() -> ActionValue {
  let get_name_argument = GetArgumentOperation::new("std:command$name").into();
  let arguments = ArrayValue::default();
  ActionValue::ExecuteCommand {
    name: get_name_argument,
    arguments,
  }
}
