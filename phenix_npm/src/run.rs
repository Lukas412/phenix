use phenix_core::{CommandOperation, GetArgumentOperation};

pub const NPM_RUN_NAME: &str = "npm:run$name";

pub fn new_npm_run_command_operation() -> CommandOperation {
  CommandOperation::from(("npm", GetArgumentOperation::new(NPM_RUN_NAME)))
}
