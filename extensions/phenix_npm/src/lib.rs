pub use {
  ext::PhenixNpmExtension,
  install::{new_npm_install, new_npm_install_packages, new_npm_install_packages_with},
  phenix_identifiers::{
    NPM, NPM_INSTALL, NPM_INSTALL__PACKAGES, NPM_RUN, NPM_RUN__ARGUMENTS, NPM_RUN__NAME,
  },
  run::{new_npm_run_command_operation, new_npm_run_with},
};

mod ext;
mod install;
mod phenix_identifiers;
mod run;
