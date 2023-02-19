use phenix_core::{ActionValue, ArrayValue};

pub(crate) fn create_command_value() -> ActionValue {
  let get_name_argument = "std:command$name".into();
  let arguments = ArrayValue::default();
  ActionValue::ExecuteCommand {
    name: get_name_argument,
    arguments,
  }
}
