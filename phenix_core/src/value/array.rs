use std::fmt::Debug;

use derive_more::From;

use crate::GetArgumentOperation;

#[derive(Clone, Debug, From)]
pub enum ArrayExpression<T>
where
  T: Clone + Debug,
{
  Value(ArrayValue<T>),
  Operation(ArrayOperation<T>),
}

impl<T> From<GetArgumentOperation<ArrayValue<T>>> for ArrayExpression<T>
where
  T: Clone + Debug,
{
  fn from(operation: GetArgumentOperation<ArrayValue<T>>) -> Self {
    Self::Operation(operation.into())
  }
}

#[derive(Clone, Debug, Default)]
pub struct ArrayValue<T>
where
  T: Clone + Debug,
{
  values: Vec<T>,
}

#[derive(Clone, Debug, From)]
pub enum ArrayOperation<T>
where
  T: Clone + Debug,
{
  #[from]
  GetArgument(GetArgumentOperation<ArrayValue<T>>),
}
