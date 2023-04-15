use crate::project::{new_init_svelte_project_command, new_setup_svelte_project_action};
use phenix_core::RuntimeBuilder;

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self
      .with_action("svelte:project:init", new_init_svelte_project_command())
      .with_action("svelte:project:create", new_setup_svelte_project_action())
  }
}
