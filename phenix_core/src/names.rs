use std::borrow::Cow;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Identifier<'a> {
  namespace: Cow<'a, Namespace<'a>>,
  name: Cow<'a, str>,
}

impl<'a> Identifier<'a> {
  pub fn new(namespace: impl Into<Cow<'a, Namespace<'a>>>, name: impl Into<Cow<'a, str>>) -> Self {
    Self {
      namespace: namespace.into(),
      name: name.into(),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Namespace<'a> {
  parts: Vec<Cow<'a, str>>,
}

impl<'a> From<&'a str> for Namespace<'a> {
  fn from(string: &'a str) -> Self {
    string.split("::").map(Cow::from).collect::<Vec<_>>().into()
  }
}

impl<'a> From<Vec<Cow<'a, str>>> for Namespace<'a> {
  fn from(parts: Vec<Cow<'a, str>>) -> Self {
    Self { parts }
  }
}
