use crate::SVELTE_COMPONENT_SCRIPT__CONTENT;
use phenix_core::{GetArgumentOperation, TextOperation};
use phenix_html::new_html_element_script_with;
use phenix_xml::new_xml_attribute_with;

pub fn new_svelte_component_script() -> TextOperation {
  new_html_element_script_with(
    new_xml_attribute_with("lang", "ts"),
    GetArgumentOperation::new(SVELTE_COMPONENT_SCRIPT__CONTENT),
  )
}
