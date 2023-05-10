pub use packages::{new_npm_install_packages, new_npm_install_packages_with};
use phenix_core::CommandValue;

mod packages;

pub fn new_npm_install() -> CommandValue {
  CommandValue::new("npm install")
}
