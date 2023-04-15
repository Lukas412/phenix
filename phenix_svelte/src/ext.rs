use phenix_core::{
  ActionOperation, CommandOperation, GetArgumentOperation, RuntimeBuilder, WordsOperation,
};

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self.with_action(
      "svelte:project:init",
      ActionOperation::from(vec![CommandOperation::new(
        "npm",
        WordsOperation::from((
          "create",
          "svelte@latest",
          GetArgumentOperation::new("svelte:project$name"),
        )),
      )
      .into()]),
    )
  }
}
