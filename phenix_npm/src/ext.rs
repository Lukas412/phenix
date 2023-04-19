use phenix_core::RuntimeBuilder;

use crate::{
  new_npm_install_blank_command_value, new_npm_install_packages_command_operation, NPM_INSTALL,
  NPM_INSTALL_BLANK,
};

pub trait PhenixNpmExtension {
  fn with_npm(self) -> Self;
}

impl PhenixNpmExtension for RuntimeBuilder {
  fn with_npm(self) -> Self {
    self
      .with_action(NPM_INSTALL_BLANK, new_npm_install_blank_command_value())
      .with_action(NPM_INSTALL, new_npm_install_packages_command_operation())
  }
}
