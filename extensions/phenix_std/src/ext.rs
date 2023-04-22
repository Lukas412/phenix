use phenix_core::RuntimeBuilder;

use crate::new_std_create_command;

pub trait RuntimeBuilderStdExt {
  fn with_std(self) -> Self;
}

impl RuntimeBuilderStdExt for RuntimeBuilder {
  fn with_std(self) -> Self {
    self
      .with_text("std:string:empty", "")
      .with_action("std:command:create", new_std_create_command())
  }
}
