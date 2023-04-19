use crate::project::SVELTE_PROJECT__NAME;
use phenix_core::{CommandOperation, GetArgumentOperation};

pub fn new_svelte_project_init_operation() -> CommandOperation {
  CommandOperation::from((
    "npm create svelte@latest",
    GetArgumentOperation::new(SVELTE_PROJECT__NAME),
  ))
}
