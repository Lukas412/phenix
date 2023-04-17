use phenix_core::{
  ActionOperation, ActionValue, CommandValue, GetArgumentOperation, LocationOperation,
  ToPathOperation,
};
use phenix_npm::new_npm_blank_install_command;

use crate::project::new_init_svelte_project_command;

pub(crate) fn new_setup_svelte_project_action() -> ActionOperation {
  ActionOperation::from((
    new_init_svelte_project_command(),
    LocationOperation::new(
      ToPathOperation::new(GetArgumentOperation::new("svelte:project$name")),
      ActionValue::from((
        new_npm_blank_install_command(),
        CommandValue::new("npm run dev -- --open"),
      )),
    ),
  ))
}
