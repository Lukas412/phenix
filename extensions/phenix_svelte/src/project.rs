pub use init::new_svelte_project_init_operation;
use phenix_core::{
  ActionOperation, ContextSwitchOperation, GetArgumentOperation, LocationOperation, ToPathOperation,
};
use phenix_npm::{
  new_npm_install_blank_command_value, new_npm_run_command_operation, NPM_RUN__ARGUMENTS,
  NPM_RUN__NAME,
};

pub const SVELTE_PROJECT: &str = "svelte:project";
pub const SVELTE_PROJECT_INIT: &str = "svelte:project:init";
pub const SVELTE_PROJECT__NAME: &str = "svelte:project$name";

mod init;

pub fn new_svelte_project_operation() -> ActionOperation {
  ActionOperation::from((
    new_svelte_project_init_operation(),
    LocationOperation::new(
      ToPathOperation::new(GetArgumentOperation::new(SVELTE_PROJECT__NAME)),
      ActionOperation::from((
        new_npm_install_blank_command_value(),
        new_npm_run_command_operation("dev", Some("--open")),
      )),
    ),
  ))
}
