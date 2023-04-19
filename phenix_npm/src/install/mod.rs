pub use {
  blank::new_npm_install_blank_command_value, packages::new_npm_install_packages_command_operation,
};

pub const NPM_INSTALL: &str = "npm:install";
pub const NPM_INSTALL_BLANK: &str = "npm:install:blank";
pub const NPM_INSTALL_PACKAGES: &str = "npm:install$packages";

mod blank;
mod packages;
