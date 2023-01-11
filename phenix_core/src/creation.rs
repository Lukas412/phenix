use std::{collections::HashMap, rc::Rc};

use crate::{BorrowedIdentifier, BorrowedNamespace, BorrowedValue};

pub type CreationArguments<'a> = Rc<HashMap<BorrowedIdentifier<'a>, Creation<'a>>>;

#[derive(Debug)]
pub enum Creation<'a> {
  Value(BorrowedValue<'a>),
  Complex {
    namespace: BorrowedNamespace<'a>,
    values: CreationArguments<'a>,
  },
}

impl<'a> From<BorrowedValue<'a>> for Creation<'a> {
  fn from(value: BorrowedValue<'a>) -> Self {
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
  namespace: BorrowedNamespace<'a>,
  values: HashMap<BorrowedIdentifier<'a>, Creation<'a>>,
}

impl<'a> ComplexCreationBuilder<'a> {
  pub fn new(namespace: BorrowedNamespace<'a>) -> Self {
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
    T: Into<BorrowedIdentifier<'a>>,
  {
    self.values.insert(name.into(), creation);
    self
  }

  pub fn with_value<T>(mut self, name: T, value: BorrowedValue<'a>) -> Self
  where
    T: Into<BorrowedIdentifier<'a>>,
  {
    self.values.insert(name.into(), value.into());
    self
  }

  pub fn with_complex<T>(mut self, name: T, complex: Self) -> Self
  where
    T: Into<BorrowedIdentifier<'a>>,
  {
    self.values.insert(name.into(), complex.into());
    self
  }
}
