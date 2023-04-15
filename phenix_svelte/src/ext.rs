use phenix_core::{
  ActionOperation, ActionValue, CommandOperation, CommandValue, GetArgumentOperation,
  LocationOperation, RuntimeBuilder, TextWordsOperation, ToPathOperation,
};

pub trait PhenixSvelteExtension {
  fn with_svelte(self) -> Self;
}

impl PhenixSvelteExtension for RuntimeBuilder {
  fn with_svelte(self) -> Self {
    self.with_action(
      "svelte:project:init",
      ActionOperation::from(vec![
        CommandOperation::new(
          "npm",
          TextWordsOperation::from((
            "create",
            "svelte@latest",
            GetArgumentOperation::new("svelte:project$name"),
          )),
        )
        .into(),
        LocationOperation::new(
          ToPathOperation::new(GetArgumentOperation::new("svelte:project$name")),
          ActionValue::from((
            CommandValue::new("npm install"),
            CommandValue::new("npm run dev -- --open"),
          )),
        )
        .into(),
      ]),
    )
  }
}
