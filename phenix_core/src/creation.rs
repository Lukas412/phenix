use std::{collections::HashMap, rc::Rc};

use crate::{Namespace, Value};

#[derive(Debug)]
pub enum Creation<'a> {
  Value(Value),
  Complex {
    namespace: Namespace<'a>,
    values: Rc<HashMap<&'a str, Creation<'a>>>,
  },
}

impl<'a> From<Value> for Creation<'a> {
  fn from(value: Value) -> Self {
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
  values: HashMap<&'a str, Creation<'a>>,
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

  pub fn with_creation(mut self, name: &'a str, creation: Creation<'a>) -> Self {
    self.values.insert(name, creation);
    self
  }

  pub fn with_value(mut self, name: &'a str, value: Value) -> Self {
    self.values.insert(name, value.into());
    self
  }

  pub fn with_complex(mut self, name: &'a str, complex: Self) -> Self {
    self.values.insert(name, complex.into());
    self
  }
}
