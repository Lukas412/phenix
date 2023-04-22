use crate::XML_ELEMENT;
use crate::{new_xml_element_empty_operation, new_xml_element_operation, XML_ELEMENT_EMPTY};
use phenix_core::RuntimeBuilder;

pub trait PhenixXmlExt {
  fn with_xml(self) -> Self;
}

impl PhenixXmlExt for RuntimeBuilder {
  fn with_xml(self) -> Self {
    self
      .with_text(XML_ELEMENT, new_xml_element_operation())
      .with_text(XML_ELEMENT_EMPTY, new_xml_element_empty_operation())
  }
}
