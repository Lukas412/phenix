use phenix_core::{ContextSwitchOperation, GetArgumentOperation, TextOperation};
use phenix_prelude::context_mapping;
use phenix_xml::{
  new_xml_element_operation, XML_ELEMENT__ARGUMENTS, XML_ELEMENT__INNER, XML_ELEMENT__NAME,
};

pub const HTML_SCRIPT: &str = "html:script";
pub const HTML_SCRIPT__ARGUMENTS: &str = "html:script$arguments";
pub const HTML_SCRIPT__CONTENT: &str = "html:script$content";

fn new_html_script_operation() -> impl Into<TextOperation> {
  new_xml_element_operation(
    "script",
    TextOperation::from(GetArgumentOperation::new(HTML_SCRIPT__ARGUMENTS)),
    TextOperation::from(GetArgumentOperation::new(HTML_SCRIPT__CONTENT)),
  )
}
