use phenix_core::{CommandExpression, RuntimeBuilder, WordsOperation};

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self.with_action(
      "svelte:project:init",
      CommandExpression::new(
        "npm",
        WordsOperation::from(vec!["create", "svelte@latest", "myapp"]),
      ),
    )
  }
}
