use phenix_core::RuntimeBuilder;

pub trait RuntimeBuilderStdExt {
  fn with_std(self) -> Self;
}

impl RuntimeBuilderStdExt for RuntimeBuilder {
  fn with_std(self) -> Self {
    self.with_string("std:string:empty", "")
  }
}
