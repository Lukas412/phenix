use phenix_core::RuntimeBuilder;

use crate::{
  new_npm_install_packages, new_npm_run_command_operation, NPM_INSTALL, NPM_RUN,
};

pub trait PhenixNpmExtension {
  fn with_npm(self) -> Self;
}

impl PhenixNpmExtension for RuntimeBuilder {
  fn with_npm(self) -> Self {
    self
      .with_action(NPM_INSTALL, new_npm_install_packages())
      .with_action(NPM_RUN, new_npm_run_command_operation())
  }
}
