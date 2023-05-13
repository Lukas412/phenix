use crate::{Creation, Identifier};
use std::collections::HashMap;

pub trait ContextExt
where
  Self: Default + Sized,
  Self: Into<DynamicContext>,
  Self: TryFrom<DynamicContext>,
{
  fn has(&self, identifier: &Identifier) -> bool;

  fn get(&self, identifier: &Identifier) -> Option<&Creation<Self>>;

  fn len(&self) -> usize;
}

#[derive(Clone, Debug, Default)]
pub struct DynamicContext(HashMap<Identifier, Creation<DynamicContext>>);

impl DynamicContext {
  pub fn extend(&mut self, other: &Self) {
    self.0.reserve(other.0.len());
    for (identifier, creation) in other.0.iter() {
      self.0.insert(identifier.clone(), creation.clone());
    }
  }
}

impl From<HashMap<Identifier, Creation<DynamicContext>>> for DynamicContext {
  fn from(value: HashMap<Identifier, Creation<DynamicContext>>) -> Self {
    Self(value)
  }
}

impl ContextExt for DynamicContext {
  fn has(&self, identifier: &Identifier) -> bool {
    self.0.contains_key(identifier)
  }

  fn get(&self, identifier: &Identifier) -> Option<&Creation<Self>> {
    self.0.get(identifier)
  }

  fn len(&self) -> usize {
    self.0.len()
  }
}
