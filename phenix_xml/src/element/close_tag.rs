use super::XML_ELEMENT_NAME;
use phenix_core::{GetArgumentOperation, TextBlockOperation, TextOperation};

pub fn new_xml_element_close_tag_operation() -> TextOperation {
  TextBlockOperation::from(("</", GetArgumentOperation::new(XML_ELEMENT_NAME), ">")).into()
}
