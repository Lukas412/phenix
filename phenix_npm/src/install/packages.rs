use phenix_core::{CommandOperation, GetArgumentOperation};

pub fn new_npm_install_packages_command() -> CommandOperation {
  CommandOperation::from((
    "npm",
    "install",
    GetArgumentOperation::new("npm:install$packages"),
  ))
}
