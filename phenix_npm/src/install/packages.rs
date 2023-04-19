use crate::NPM_INSTALL_PACKAGES;
use phenix_core::{CommandOperation, GetArgumentOperation};

pub fn new_npm_install_packages_command_operation() -> CommandOperation {
  CommandOperation::from((
    "npm install",
    GetArgumentOperation::new(NPM_INSTALL_PACKAGES),
  ))
}
