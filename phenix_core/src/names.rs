use std::borrow::Cow;

pub type Name<'a> = Cow<'a, str>;
pub type OwnedName = String;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Identifier<'a> {
  namespace: Namespace<'a>,
  name: Name<'a>,
}

impl<'a> Identifier<'a> {
  pub fn new(namespace: Namespace<'a>, name: Name<'a>) -> Self {
    Self { namespace, name }
  }
}

impl<'a, T, U> From<(T, U)> for Identifier<'a>
where
  T: Into<Namespace<'a>>,
  U: Into<Name<'a>>,
{
  fn from(values: (T, U)) -> Self {
    Self::new(values.0.into(), values.1.into())
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OwnedIdentifier {
  namespace: OwnedNamespace,
  name: OwnedName,
}

impl OwnedIdentifier {
  pub fn new(namespace: OwnedNamespace, name: OwnedName) -> Self {
    Self { namespace, name }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Namespace<'a> {
  parts: Vec<Name<'a>>,
}

impl<'a> From<&'a str> for Namespace<'a> {
  fn from(string: &'a str) -> Self {
    string
      .split("::")
      .map(Name::from)
      .collect::<Vec<_>>()
      .into()
  }
}

impl<'a> From<Vec<Name<'a>>> for Namespace<'a> {
  fn from(parts: Vec<Name<'a>>) -> Self {
    Self { parts }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OwnedNamespace {
  parts: Vec<OwnedName>,
}

impl From<String> for OwnedNamespace {
  fn from(string: String) -> Self {
    string
      .split("::")
      .map(ToOwned::to_owned)
      .collect::<Vec<_>>()
      .into()
  }
}

impl From<Vec<OwnedName>> for OwnedNamespace {
  fn from(parts: Vec<OwnedName>) -> Self {
    Self { parts }
  }
}
