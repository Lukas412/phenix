use crate::element::{XML_ELEMENT__ATTRIBUTES, XML_ELEMENT__NAME};
use phenix_core::{GetArgumentOperation, TextBlockOperation, TextOperation, TextWordsOperation};

pub fn new_xml_element_open_tag_operation() -> TextOperation {
  TextBlockOperation::from((
    "<",
    TextWordsOperation::from((
      GetArgumentOperation::new(XML_ELEMENT__NAME),
      GetArgumentOperation::new(XML_ELEMENT__ATTRIBUTES),
    )),
    ">",
  ))
  .into()
}
