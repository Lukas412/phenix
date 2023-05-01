use phenix_core::ActionOperation;

pub use {
  content::new_svelte_component_content, script::new_svelte_component_script,
  style::new_svelte_component_style,
};

mod content;
mod script;
mod style;

pub fn new_svelte_component() -> ActionOperation {
  vec![].into()
}
