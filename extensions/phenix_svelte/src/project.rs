pub use init::new_svelte_project_init_operation;
use phenix_core::{ActionOperation, GetArgumentOperation, LocationOperation, ToPathOperation};
use phenix_npm::{
  new_npm_install_blank_command_value, new_npm_run_command_operation_with_context_switch,
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
        new_npm_run_command_operation_with_context_switch("dev", Some("--open")),
      )),
    ),
  ))
}
