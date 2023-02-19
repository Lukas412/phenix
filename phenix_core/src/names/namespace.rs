use std::fmt::{Display};

use super::Name;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Namespace {
  parts: Vec<Name>,
}

impl Namespace {
  const SEPARATOR: &str = ":";
}

impl Display for Namespace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.parts.join(Self::SEPARATOR))
  }
}

impl From<&str> for Namespace {
  fn from(string: &str) -> Self {
    string
      .split(Self::SEPARATOR)
      .map(Name::from)
      .collect::<Vec<_>>()
      .into()
  }
}

impl From<Vec<Name>> for Namespace {
  fn from(parts: Vec<Name>) -> Self {
    Self { parts }
  }
}
