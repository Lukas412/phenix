use crate::{HTML_ELEMENT_SCRIPT__ARGUMENTS, HTML_ELEMENT_SCRIPT__CONTENT};
use phenix_core::{
  ContextSwitchOperation, Creation, DynamicContext, GetArgumentOperation, TextOperation,
};
use phenix_xml::new_xml_element_with;

pub fn new_html_element_script_with(
  arguments: impl Into<TextOperation>,
  content: impl Into<TextOperation>,
) -> TextOperation {
  ContextSwitchOperation::new(
    new_html_element_script_context(arguments.into(), content.into()),
    new_html_element_script(),
  )
  .into()
}

pub fn new_html_element_script() -> TextOperation {
  new_xml_element_with(
    "script",
    GetArgumentOperation::new(HTML_ELEMENT_SCRIPT__ARGUMENTS),
    GetArgumentOperation::new(HTML_ELEMENT_SCRIPT__CONTENT),
  )
}

fn new_html_element_script_context(
  arguments: impl Into<Creation>,
  content: impl Into<Creation>,
) -> DynamicContext {
  [
    (HTML_ELEMENT_SCRIPT__ARGUMENTS.into(), arguments.into()),
    (HTML_ELEMENT_SCRIPT__CONTENT.into(), content.into()),
  ]
  .into()
}
