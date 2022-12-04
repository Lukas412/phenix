use std::{collections::HashMap, rc::Rc};

use crate::{Identifier, Namespace, Value};

pub type CreationArguments<'a> = Rc<HashMap<Identifier<'a>, Creation<'a>>>;

#[derive(Debug)]
pub enum Creation<'a> {
  Value(Value<'a>),
  Complex {
    namespace: Namespace<'a>,
    values: CreationArguments<'a>,
  },
}

impl<'a> From<Value<'a>> for Creation<'a> {
  fn from(value: Value<'a>) -> Self {
    Self::Value(value)
  }
}

impl<'a> From<ComplexCreationBuilder<'a>> for Creation<'a> {
  fn from(builder: ComplexCreationBuilder<'a>) -> Self {
    Self::Complex {
      namespace: builder.namespace,
      values: builder.values.into(),
    }
  }
}

#[derive(Debug)]
pub struct ComplexCreationBuilder<'a> {
  namespace: Namespace<'a>,
  values: HashMap<Identifier<'a>, Creation<'a>>,
}

impl<'a> ComplexCreationBuilder<'a> {
  pub fn new(namespace: Namespace<'a>) -> Self {
    Self {
      namespace,
      values: HashMap::new(),
    }
  }

  pub fn build(self) -> Creation<'a> {
    self.into()
  }

  pub fn with_creation<T>(mut self, name: T, creation: Creation<'a>) -> Self
  where
    T: Into<Identifier<'a>>,
  {
    self.values.insert(name.into(), creation);
    self
  }

  pub fn with_value<T>(mut self, name: T, value: Value<'a>) -> Self
  where
    T: Into<Identifier<'a>>,
  {
    self.values.insert(name.into(), value.into());
    self
  }

  pub fn with_complex<T>(mut self, name: T, complex: Self) -> Self
  where
    T: Into<Identifier<'a>>,
  {
    self.values.insert(name.into(), complex.into());
    self
  }
}
