use phenix_core::{
  ActionExpression, CommandExpression, GetArgumentOperation, RuntimeBuilder, WordsOperation,
};

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self.with_action(
      "svelte:project:init",
      ActionExpression::from(vec![ActionExpression::from(CommandExpression::new(
        "npm",
        WordsOperation::from((
          "create",
          "svelte@latest",
          GetArgumentOperation::new("svelte:project$name"),
        )),
      ))]),
    )
  }
}
