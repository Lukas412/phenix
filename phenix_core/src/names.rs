#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Identifier<'a> {
  namespace: Namespace<'a>,
  name: &'a str,
}

impl<'a> Identifier<'a> {
  pub fn new(namespace: Namespace<'a>, name: &'a str) -> Self {
    Self { namespace, name }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Namespace<'a> {
  parts: Vec<&'a str>,
}

impl<'a> From<&'a str> for Namespace<'a> {
  fn from(string: &'a str) -> Self {
    string.split("::").collect::<Vec<&'a str>>().into()
  }
}

impl<'a> From<Vec<&'a str>> for Namespace<'a> {
  fn from(parts: Vec<&'a str>) -> Self {
    Self { parts }
  }
}
