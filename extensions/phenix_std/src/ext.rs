use phenix_core::RuntimeBuilder;

use crate::{new_std_create_command, new_std_text_empty, STD_TEXT_EMPTY};

pub trait RuntimeBuilderStdExt {
  fn with_std(self) -> Self;
}

impl RuntimeBuilderStdExt for RuntimeBuilder {
  fn with_std(self) -> Self {
    self
      .with_text(STD_TEXT_EMPTY, new_std_text_empty())
      .with_action("std:command:create", new_std_create_command())
  }
}
