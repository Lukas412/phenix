use crate::{Creation, Identifier, Namespace};
use std::collections::HashMap;
use std::rc::Rc;

pub type ComplexCreationArguments = Rc<HashMap<Identifier, Creation>>;

#[derive(Clone, Debug)]
pub struct ComplexCreation {
  namespace: Namespace,
  values: ComplexCreationArguments,
}

impl ComplexCreation {
  fn new(namespace: Namespace, values: ComplexCreationArguments) -> Self {
    Self { namespace, values }
  }

  pub(crate) fn namespace(&self) -> &Namespace {
    &self.namespace
  }

  pub(crate) fn values(&self) -> &ComplexCreationArguments {
    &self.values
  }
}

#[derive(Clone, Debug)]
pub struct ComplexCreationBuilder {
  namespace: Namespace,
  values: HashMap<Identifier, Creation>,
}

impl ComplexCreationBuilder {
  pub fn new<N>(namespace: N) -> Self
  where
    N: Into<Namespace>,
  {
    Self {
      namespace: namespace.into(),
      values: HashMap::new(),
    }
  }

  pub fn build(self) -> ComplexCreation {
    ComplexCreation::new(self.namespace, self.values.into())
  }

  pub fn with<T, C>(mut self, identifier: T, creation: C) -> Self
  where
    T: Into<Identifier>,
    C: Into<Creation>,
  {
    self.values.insert(identifier.into(), creation.into());
    self
  }
}