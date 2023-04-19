use phenix_core::{GetArgumentOperation, TextBlockOperation, TextOperation};

pub use {
  close_tag::new_xml_element_close_tag_operation, empty::new_xml_element_empty_operation,
  open_tag::new_xml_element_open_tag_operation,
};

mod close_tag;
mod empty;
mod open_tag;

pub const XML_ELEMENT_NAME: &str = "xml:element$name";
pub const XML_ELEMENT_ARGUMENTS: &str = "xml:element$arguments";
pub const XML_ELEMENT_INNER: &str = "xml:element$inner";

pub fn new_xml_element_operation() -> TextOperation {
  TextBlockOperation::from((
    new_xml_element_open_tag_operation(),
    GetArgumentOperation::new(XML_ELEMENT_INNER),
    new_xml_element_close_tag_operation(),
  ))
  .into()
}
