use crate::{new_html_script_operation, HTML_SCRIPT};
use phenix_core::RuntimeBuilder;

pub trait PhenixHtmlExt {
  fn with_html(self) -> Self;
}

impl PhenixHtmlExt for RuntimeBuilder {
  fn with_html(self) -> Self {
    self.with_text(HTML_SCRIPT, new_html_script_operation())
  }
}
