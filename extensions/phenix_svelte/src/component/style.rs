use crate::SVELTE_COMPONENT_STYLE__CONTENT;
use phenix_core::{GetArgumentOperation, TextOperation};
use phenix_html::new_html_element_style_with;
use phenix_std::new_std_text_empty;

pub fn new_svelte_component_style() -> TextOperation {
  new_html_element_style_with(
    new_std_text_empty(),
    GetArgumentOperation::new(SVELTE_COMPONENT_STYLE__CONTENT),
  )
}
