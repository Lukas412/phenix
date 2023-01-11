use crate::{BorrowedName, BorrowedNamespace};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BorrowedIdentifier<'a> {
  namespace: BorrowedNamespace<'a>,
  name: BorrowedName<'a>,
}

impl<'a> BorrowedIdentifier<'a> {
  fn new(namespace: BorrowedNamespace<'a>, name: BorrowedName<'a>) -> Self {
    Self { namespace, name }
  }
}

impl<'a, T, U> From<(T, U)> for BorrowedIdentifier<'a>
where
  T: Into<BorrowedNamespace<'a>>,
  U: Into<BorrowedName<'a>>,
{
  fn from(values: (T, U)) -> Self {
    Self::new(values.0.into(), values.1.into())
  }
}
