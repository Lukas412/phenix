use phenix_core::ActionOperation;

mod body;
mod content;
mod script;
mod style;

pub fn new_svelte_page_operation() -> ActionOperation {
  vec![].into()
}
