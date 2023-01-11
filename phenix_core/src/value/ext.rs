use crate::{BorrowedValue, CreationArguments, Runtime};

use super::ConcreteValue;

pub trait ValueExt: Clone {
  fn eval<'a>(
    &'a self,
    runtime: &'a Runtime,
    arguments: CreationArguments<'a>,
  ) -> Result<BorrowedValue<'static>, String>;

  fn get_concrete(&self) -> Option<ConcreteValue>;
}
