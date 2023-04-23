use crate::SVELTE_PROJECT;
use crate::{new_svelte_project_init_operation, new_svelte_project_operation, SVELTE_PROJECT_INIT};
use phenix_core::RuntimeBuilder;

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self
      .with_action(SVELTE_PROJECT_INIT, new_svelte_project_init_operation())
      .with_action(SVELTE_PROJECT, new_svelte_project_operation())
  }
}
