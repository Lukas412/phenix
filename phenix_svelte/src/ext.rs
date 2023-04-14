use phenix_core::{ActionValue, CommandValue, RuntimeBuilder};

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self.with_action(
      "svelte:project:init",
      vec![ActionValue::from(CommandValue::new("svelte", vec![]))],
    )
  }
}
