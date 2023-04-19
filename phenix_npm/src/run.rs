use phenix_core::{CommandOperation, GetArgumentOperation};

pub const NPM_RUN: &str = "npm:run";
pub const NPM_RUN__NAME: &str = "npm:run$name";
pub const NPM_RUN__ARGUMENTS: &str = "npm:run$arguments";

pub fn new_npm_run_command_operation() -> CommandOperation {
  CommandOperation::from(("npm", GetArgumentOperation::new(NPM_RUN__NAME)))
}
