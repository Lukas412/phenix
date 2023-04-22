use phenix_core::{GetArgumentOperation, TextBlockOperation, TextOperation};

use super::XML_ELEMENT__NAME;

pub fn new_xml_element_empty_operation() -> TextOperation {
  TextBlockOperation::from(("<", GetArgumentOperation::new(XML_ELEMENT__NAME), "/>")).into()
}
