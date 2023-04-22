use phenix_core::RuntimeBuilder;

pub trait PhenixHtmlExt {
  fn with_html(self) -> Self;
}

impl PhenixHtmlExt for RuntimeBuilder {
  fn with_html(self) -> Self {
    self
  }
}