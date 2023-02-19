use phenix_core::RuntimeBuilder;

use crate::command::create_command_value;

pub trait RuntimeBuilderStdExt {
  fn with_std(self) -> Self;
}

impl RuntimeBuilderStdExt for RuntimeBuilder {
  fn with_std(self) -> Self {
    self
      .with_string("std:string:empty", "")
      .with_action("std:command:create", create_command_value())
  }
}
