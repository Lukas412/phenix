use phenix_core::RuntimeBuilder;

use crate::{new_npm_blank_install_command, new_npm_install_packages_command};

pub trait PhenixNpmExtension {
  fn with_npm(self) -> Self;
}

impl PhenixNpmExtension for RuntimeBuilder {
  fn with_npm(self) -> Self {
    self
      .with_action("npm:install:blank", new_npm_blank_install_command())
      .with_action("npm:install:packages", new_npm_install_packages_command())
  }
}
