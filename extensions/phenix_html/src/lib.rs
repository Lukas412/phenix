pub use {
  elements::{
    new_html_element_script, new_html_element_script_with, new_html_element_style,
    new_html_element_style_with,
  },
  ext::PhenixHtmlExt,
  phenix_identifiers::{
    HTML, HTML_ELEMENT, HTML_ELEMENT_SCRIPT, HTML_ELEMENT_SCRIPT__ARGUMENTS,
    HTML_ELEMENT_SCRIPT__CONTENT, HTML_ELEMENT_STYLE, HTML_ELEMENT_STYLE__ARGUMENTS,
    HTML_ELEMENT_STYLE__CONTENT,
  },
};

mod elements;
mod ext;
mod phenix_identifiers;
