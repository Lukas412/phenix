use phenix_core::{
  ContextSwitchOperation, Creation, EvaluateArguments, GetArgumentOperation, TextOperation,
};
use phenix_xml::new_xml_element_operation_with_context_switch;

pub const HTML_ELEMENT_SCRIPT: &str = "html:element:script";
pub const HTML_ELEMENT_SCRIPT__ARGUMENTS: &str = "html:element:script$arguments";
pub const HTML_ELEMENT_SCRIPT__CONTENT: &str = "html:element:script$content";

pub fn new_html_element_script_with(
  arguments: impl Into<TextOperation>,
  content: impl Into<TextOperation>,
) -> TextOperation {
  ContextSwitchOperation::new(
    new_html_element_script_context(arguments.into(), content.into()),
    new_html_element_script_operation(),
  )
  .into()
}

pub fn new_html_element_script_operation() -> TextOperation {
  new_xml_element_operation_with_context_switch(
    "script",
    TextOperation::from(GetArgumentOperation::new(HTML_ELEMENT_SCRIPT__ARGUMENTS)),
    TextOperation::from(GetArgumentOperation::new(HTML_ELEMENT_SCRIPT__CONTENT)),
  )
}

fn new_html_element_script_context(
  arguments: impl Into<Creation>,
  content: impl Into<Creation>,
) -> EvaluateArguments {
  [
    (HTML_ELEMENT_SCRIPT__ARGUMENTS.into(), arguments.into()),
    (HTML_ELEMENT_SCRIPT__CONTENT.into(), content.into()),
  ]
  .into()
}
