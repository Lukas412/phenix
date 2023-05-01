pub use packages::{new_npm_install_packages, new_npm_install_packages_with};
use phenix_core::CommandValue;

pub const NPM_INSTALL: &str = "npm:install";
pub const NPM_INSTALL__PACKAGES: &str = "npm:install$packages";

mod packages;

pub fn new_npm_install() -> CommandValue {
  CommandValue::new("npm install")
}
