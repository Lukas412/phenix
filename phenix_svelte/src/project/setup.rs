use crate::project::new_init_svelte_project_command;
use phenix_core::{
  ActionOperation, ActionValue, CommandValue, GetArgumentOperation, LocationOperation,
  ToPathOperation,
};

pub(crate) fn new_setup_svelte_project_action() -> ActionOperation {
  ActionOperation::from((
    new_init_svelte_project_command(),
    LocationOperation::new(
      ToPathOperation::new(GetArgumentOperation::new("svelte:project$name")),
      ActionValue::from((
        CommandValue::new("npm install"),
        CommandValue::new("npm run dev -- --open"),
      )),
    ),
  ))
}
