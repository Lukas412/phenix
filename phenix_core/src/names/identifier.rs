use derive_more::Display;
use std::cmp::min;

use super::{Name, Namespace};

#[derive(Clone, Debug, Default, Display, PartialEq, Eq, Hash)]
#[display(fmt = "{namespace}${name}")]
pub struct Identifier {
  namespace: Namespace,
  name: Name,
}

impl Identifier {
  const SEPARATOR: &'static str = "$";

  fn new(namespace: Namespace, name: Name) -> Self {
    Self { namespace, name }
  }
}

impl From<&str> for Identifier {
  fn from(value: &str) -> Self {
    value
      .find(Self::SEPARATOR)
      .map(|index| {
        let name_start_index_in_bounds = min(index + 1, value.len());
        (&value[..index], &value[name_start_index_in_bounds..]).into()
      })
      .unwrap_or_default()
  }
}

impl<T, U> From<(T, U)> for Identifier
where
  T: Into<Namespace>,
  U: Into<Name>,
{
  fn from(values: (T, U)) -> Self {
    Self::new(values.0.into(), values.1.into())
  }
}
