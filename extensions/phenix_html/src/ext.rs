use crate::{
  new_html_element_script, new_html_element_style, HTML_ELEMENT_SCRIPT, HTML_ELEMENT_STYLE,
};
use phenix_core::RuntimeBuilder;

pub trait PhenixHtmlExt {
  fn with_html(self) -> Self;
}

impl PhenixHtmlExt for RuntimeBuilder {
  fn with_html(self) -> Self {
    self
      .with_text(HTML_ELEMENT_SCRIPT, new_html_element_script())
      .with_text(HTML_ELEMENT_STYLE, new_html_element_style())
  }
}
