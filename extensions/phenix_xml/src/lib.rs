pub use {
  attribute::{new_xml_attribute_operation, new_xml_attribute_with},
  element::{
    new_xml_element_close_tag_operation, new_xml_element_empty_operation,
    new_xml_element_open_tag_operation, new_xml_element_operation,
    new_xml_element_operation_with_context_switch, XML_ELEMENT, XML_ELEMENT_EMPTY,
    XML_ELEMENT__ATTRIBUTES, XML_ELEMENT__INNER, XML_ELEMENT__NAME,
  },
  ext::PhenixXmlExt,
  phenix_identifiers::{XML_ATTRIBUTE, XML_ATTRIBUTE__NAME, XML_ATTRIBUTE__VALUE},
};

mod attribute;
mod element;
mod ext;
mod phenix_identifiers;
