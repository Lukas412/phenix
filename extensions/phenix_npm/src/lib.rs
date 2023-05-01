pub use {
  ext::PhenixNpmExtension,
  install::{
    new_npm_install, new_npm_install_packages, new_npm_install_packages_with, NPM_INSTALL,
    NPM_INSTALL__PACKAGES,
  },
  run::{
    new_npm_run_command_operation, new_npm_run_command_with, NPM_RUN, NPM_RUN__ARGUMENTS,
    NPM_RUN__NAME,
  },
};

mod ext;
mod install;
mod phenix_identifiers;
mod run;
