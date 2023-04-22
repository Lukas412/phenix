use crate::{new_html_element_script_operation, HTML_ELEMENT_SCRIPT};
use phenix_core::RuntimeBuilder;

pub trait PhenixHtmlExt {
  fn with_html(self) -> Self;
}

impl PhenixHtmlExt for RuntimeBuilder {
  fn with_html(self) -> Self {
    self.with_text(HTML_ELEMENT_SCRIPT, new_html_element_script_operation())
  }
}
