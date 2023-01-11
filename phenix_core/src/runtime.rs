use std::collections::HashMap;

use crate::{BorrowedNamespace, BorrowedValue, Creation, ValueExt};

#[derive(Debug, Default)]
pub struct Runtime<'a> {
  values: HashMap<BorrowedNamespace<'a>, BorrowedValue<'a>>,
}

impl<'a> Runtime<'a> {
  pub fn eval<'b>(&'b self, creation: &'b Creation<'b>) -> Result<BorrowedValue<'static>, String> {
    match creation {
      Creation::Value(value) => Ok(value.clone().into()),
      Creation::Complex { namespace, values } => {
        let value = self
          .get_value(&namespace)
          .ok_or_else(|| "value not found".to_owned())?;

        value.eval(self, values.clone())
      }
    }
  }

  fn get_value<'b>(&'a self, namespace: &'b BorrowedNamespace<'a>) -> Option<&'a BorrowedValue> {
    self.values.get(namespace)
  }
}

impl<'a> From<RuntimeBuilder<'a>> for Runtime<'a> {
  fn from(builder: RuntimeBuilder<'a>) -> Self {
    Self {
      values: builder.values,
    }
  }
}

#[derive(Default)]
pub struct RuntimeBuilder<'a> {
  values: HashMap<BorrowedNamespace<'a>, BorrowedValue<'a>>,
}

impl<'a> RuntimeBuilder<'a> {
  pub fn build(self) -> Runtime<'a> {
    self.into()
  }

  pub fn with_value(mut self, namespace: BorrowedNamespace<'a>, value: BorrowedValue<'a>) -> Self {
    self.values.insert(namespace, value);
    self
  }
}
