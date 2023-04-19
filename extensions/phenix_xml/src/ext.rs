use phenix_core::RuntimeBuilder;

pub trait PhenixXmlExt {
  fn with_xml(self) -> Self;
}

impl PhenixXmlExt for RuntimeBuilder {
  fn with_xml(self) -> Self {
    self
  }
}