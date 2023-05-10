use crate::{HTML_ELEMENT_STYLE__ARGUMENTS, HTML_ELEMENT_STYLE__CONTENT};
use phenix_core::{
  ContextSwitchOperation, Creation, DynamicContext, GetArgumentOperation, TextExpression,
  TextOperation,
};
use phenix_xml::new_xml_element_with;

pub fn new_html_element_style_with(
  arguments: impl Into<TextExpression>,
  content: impl Into<TextExpression>,
) -> TextOperation {
  ContextSwitchOperation::new(
    new_html_element_script_context(arguments.into(), content.into()),
    new_html_element_style(),
  )
  .into()
}

pub fn new_html_element_style() -> TextOperation {
  new_xml_element_with(
    "script",
    GetArgumentOperation::new(HTML_ELEMENT_STYLE__ARGUMENTS),
    GetArgumentOperation::new(HTML_ELEMENT_STYLE__CONTENT),
  )
}

fn new_html_element_script_context(
  arguments: impl Into<Creation>,
  content: impl Into<Creation>,
) -> DynamicContext {
  [
    (HTML_ELEMENT_STYLE__ARGUMENTS.into(), arguments.into()),
    (HTML_ELEMENT_STYLE__CONTENT.into(), content.into()),
  ]
  .into()
}
