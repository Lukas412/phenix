use phenix_core::{ActionValue, ArrayValue, GetArgumentOperation, StringOperation};

pub fn create_command_value() -> ActionValue {
  let get_argument = GetArgumentOperation::new(("std::command", "name"));
  let operation = StringOperation::GetArgument(get_argument).into();
  let arguments = ArrayValue::default();
  ActionValue::ExecuteCommand {
    name: operation,
    arguments,
  }
}
