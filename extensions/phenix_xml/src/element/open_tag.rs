use crate::element::{XML_ELEMENT_ARGUMENTS, XML_ELEMENT_NAME};
use phenix_core::{GetArgumentOperation, TextBlockOperation, TextOperation, TextWordsOperation};

pub fn new_xml_element_open_tag_operation() -> TextOperation {
  TextBlockOperation::from((
    "<",
    TextWordsOperation::from((
      GetArgumentOperation::new(XML_ELEMENT_NAME),
      GetArgumentOperation::new(XML_ELEMENT_ARGUMENTS),
    )),
    ">",
  ))
  .into()
}
