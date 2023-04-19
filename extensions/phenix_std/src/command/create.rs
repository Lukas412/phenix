use phenix_core::{CommandOperation, GetArgumentOperation};

pub fn new_std_create_command() -> CommandOperation {
  CommandOperation::from((
    GetArgumentOperation::new("std:command$name"),
    GetArgumentOperation::new("std:command$arguments"),
  ))
}
