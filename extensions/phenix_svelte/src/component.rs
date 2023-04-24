use phenix_core::ActionOperation;

pub use {content::new_svelte_component_content, script::new_svelte_component_script};

mod content;
mod script;
mod style;

pub fn new_svelte_component() -> ActionOperation {
  vec![].into()
}
