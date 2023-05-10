use std::collections::HashMap;

use crate::{Creation, DynamicContext, Identifier, Namespace};

#[derive(Clone, Debug)]
pub struct ComplexCreation {
  namespace: Namespace,
  values: DynamicContext,
}

impl ComplexCreation {
  fn new(namespace: Namespace, values: DynamicContext) -> Self {
    Self { namespace, values }
  }

  pub(crate) fn namespace(&self) -> &Namespace {
    &self.namespace
  }

  pub(crate) fn values(&self) -> &DynamicContext {
    &self.values
  }
}

#[derive(Clone, Debug)]
pub struct ComplexCreationBuilder {
  namespace: Namespace,
  values: HashMap<Identifier, Creation>,
}

impl ComplexCreationBuilder {
  pub fn new<IntoNamespace>(namespace: IntoNamespace) -> Self
  where
    IntoNamespace: Into<Namespace>,
  {
    Self {
      namespace: namespace.into(),
      values: HashMap::new(),
    }
  }

  pub fn with<IntoIdentifier, IntoCreation>(
    mut self,
    identifier: IntoIdentifier,
    creation: IntoCreation,
  ) -> Self
  where
    IntoIdentifier: Into<Identifier>,
    IntoCreation: Into<Creation>,
  {
    self.values.insert(identifier.into(), creation.into());
    self
  }
}

impl From<ComplexCreationBuilder> for ComplexCreation {
  fn from(value: ComplexCreationBuilder) -> Self {
    Self::new(value.namespace, value.values)
  }
}
