use phenix_core::RuntimeBuilder;

pub trait GitExt {
  fn with_git(self) -> Self;
}

impl GitExt for RuntimeBuilder {
  fn with_git(self) -> Self {
    self
  }
}
