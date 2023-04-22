use crate::new_xml_element_operation;
use crate::XML_ELEMENT;
use phenix_core::RuntimeBuilder;

pub trait PhenixXmlExt {
  fn with_xml(self) -> Self;
}

impl PhenixXmlExt for RuntimeBuilder {
  fn with_xml(self) -> Self {
    self.with_text(XML_ELEMENT, new_xml_element_operation())
  }
}
