use crate::BorrowedName;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BorrowedNamespace<'a> {
  parts: Vec<BorrowedName<'a>>,
}

impl<'a> From<BorrowedName<'a>> for BorrowedNamespace<'a> {
  fn from(string: BorrowedName<'a>) -> Self {
    string
      .split("::")
      .map(BorrowedName::from)
      .collect::<Vec<_>>()
      .into()
  }
}

impl<'a> From<Vec<BorrowedName<'a>>> for BorrowedNamespace<'a> {
  fn from(parts: Vec<BorrowedName<'a>>) -> Self {
    Self { parts }
  }
}
