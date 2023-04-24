use crate::component::script::new_svelte_component_script;
use crate::SVELTE_COMPONENT__BODY;
use phenix_core::{GetArgumentOperation, TextWordsOperation};

pub fn new_svelte_component_content() -> TextWordsOperation {
  (
    new_svelte_component_script(),
    GetArgumentOperation::new(SVELTE_COMPONENT__BODY),
  )
    .into()
}
