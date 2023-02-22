use std::fmt::Debug;

use derive_more::From;

use crate::{Expression, GetArgumentOperation};

pub type ArrayExpression<T> = Expression<ArrayValue<T>, ArrayOperation<T>>;

impl<T> From<ArrayOperation<T>> for ArrayExpression<T>
where T: Clone + Debug
{
  fn from(operation: ArrayOperation<T>) -> Self {
    ArrayExpression::Operation(Box::new(operation))
  }
}

impl<T> From<GetArgumentOperation<ArrayValue<T>>> for ArrayExpression<T>
where T: Clone + Debug
{
  fn from(operation: GetArgumentOperation<ArrayValue<T>>) -> Self {
    ArrayExpression::Operation(Box::new(operation.into()))
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
  where T: Clone + Debug
{
  #[from]
  GetArgument(GetArgumentOperation<ArrayValue<T>>),
}
