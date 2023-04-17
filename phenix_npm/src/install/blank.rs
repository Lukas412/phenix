use phenix_core::CommandValue;

pub fn new_npm_blank_install_command() -> CommandValue {
  CommandValue::new("npm install")
}
