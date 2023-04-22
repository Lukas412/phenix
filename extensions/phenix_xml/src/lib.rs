pub use {
  element::{
    new_xml_element_close_tag_operation, new_xml_element_empty_operation,
    new_xml_element_open_tag_operation, new_xml_element_operation,
    new_xml_element_operation_with_context_switch, XML_ELEMENT, XML_ELEMENT__ARGUMENTS,
    XML_ELEMENT__INNER, XML_ELEMENT__NAME,
  },
  ext::PhenixXmlExt,
};

mod element;
mod ext;
