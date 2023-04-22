use phenix_core::{GetArgumentOperation, TextOperation};
use phenix_xml::new_xml_element_operation_with_context_switch;

pub const HTML_SCRIPT: &str = "html:script";
pub const HTML_SCRIPT__ARGUMENTS: &str = "html:script$arguments";
pub const HTML_SCRIPT__CONTENT: &str = "html:script$content";

pub fn new_html_script_operation() -> impl Into<TextOperation> {
  new_xml_element_operation_with_context_switch(
    "script",
    TextOperation::from(GetArgumentOperation::new(HTML_SCRIPT__ARGUMENTS)),
    TextOperation::from(GetArgumentOperation::new(HTML_SCRIPT__CONTENT)),
  )
}
