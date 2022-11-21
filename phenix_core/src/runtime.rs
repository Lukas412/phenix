use std::collections::HashMap;

use crate::{Creation, Namespace, Value, ValueExt};

#[derive(Debug, Default)]
pub struct Runtime<'a> {
  values: HashMap<Namespace<'a>, Value>,
}

impl<'a> Runtime<'a> {
  pub fn eval(&self, creation: Creation) -> Result<Value, String> {
    match creation {
      Creation::Value(value) => Ok(value),
      Creation::Complex { namespace, values } => {
        let value = self
          .get_value(&namespace)
          .ok_or_else(|| "value not found".to_owned())?;

        value.eval(values)
      }
    }
  }

  fn get_value(&'a self, namespace: &'a Namespace) -> Option<&'a Value> {
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
  values: HashMap<Namespace<'a>, Value>,
}

impl<'a> RuntimeBuilder<'a> {
  pub fn build(self) -> Runtime<'a> {
    self.into()
  }

  pub fn with_value(mut self, namespace: Namespace<'a>, value: Value) -> Self {
    self.values.insert(namespace, value);
    self
  }
}
