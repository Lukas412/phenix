use phenix_core::{
  ActionOperation, CommandExpression, GetArgumentOperation, RuntimeBuilder, WordsOperation,
};

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self.with_action(
      "svelte:project:init",
      ActionOperation::from(vec![CommandExpression::new(
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
