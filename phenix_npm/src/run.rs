use phenix_core::{CommandOperation, GetArgumentOperation};

pub fn new_npm_run_command() -> CommandOperation {
  CommandOperation::from(("npm", GetArgumentOperation::new("npm:run$script")))
}
