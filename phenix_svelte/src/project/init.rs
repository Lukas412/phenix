use phenix_core::{CommandOperation, GetArgumentOperation, TextWordsOperation};

pub(crate) fn new_init_svelte_project_command() -> CommandOperation {
  CommandOperation::new(
    "npm",
    TextWordsOperation::from((
      "create",
      "svelte@latest",
      GetArgumentOperation::new("svelte:project$name"),
    )),
  )
}
