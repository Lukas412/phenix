use phenix_core::{
  ContextSwitchOperation, Creation, EvaluateArguments, GetArgumentOperation, Identifier,
  TextBlockOperation, TextOperation,
};
pub use {
  close_tag::new_xml_element_close_tag_operation, empty::new_xml_element_empty_operation,
  open_tag::new_xml_element_open_tag_operation,
};

mod close_tag;
mod empty;
mod open_tag;

pub const XML_ELEMENT: &str = "xml:element";
pub const XML_ELEMENT__NAME: &str = "xml:element$name";
pub const XML_ELEMENT__ARGUMENTS: &str = "xml:element$arguments";
pub const XML_ELEMENT__INNER: &str = "xml:element$inner";

pub fn new_xml_element_operation_with_context_switch(
  name: impl Into<Creation>,
  arguments: impl Into<Creation>,
  inner: impl Into<Creation>,
) -> TextOperation {
  ContextSwitchOperation::new(
    new_xml_element_context(name, arguments, inner),
    new_xml_element_operation(),
  )
  .into()
}

pub fn new_xml_element_operation() -> TextBlockOperation {
  (
    new_xml_element_open_tag_operation(),
    GetArgumentOperation::new(XML_ELEMENT__INNER),
    new_xml_element_close_tag_operation(),
  )
    .into()
}

fn new_xml_element_context(
  name: impl Into<Creation>,
  arguments: impl Into<Creation>,
  inner: impl Into<Creation>,
) -> impl Into<EvaluateArguments> {
  [
    (XML_ELEMENT__NAME.into(), name.into()),
    (XML_ELEMENT__ARGUMENTS.into(), arguments.into()),
    (XML_ELEMENT__INNER.into(), inner.into()),
  ]
}
