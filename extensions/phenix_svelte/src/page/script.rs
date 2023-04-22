use phenix_core::{GetArgumentOperation, TextOperation};
use phenix_html::new_html_element_script_with;
use phenix_xml::new_xml_attribute_with;

pub const SVELTE_PAGE_SCRIPT__CONTENT: &str = "svelte:page:script__content";

pub fn new_svelte_page_script() -> TextOperation {
  new_html_element_script_with(
    new_xml_attribute_with("lang", "ts"),
    GetArgumentOperation::new(SVELTE_PAGE_SCRIPT__CONTENT),
  )
}
