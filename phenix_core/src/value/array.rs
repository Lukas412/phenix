use std::fmt::Debug;

use crate::{Expression, GetArgumentOperation};

pub type ArrayExpression<T> = Expression<ArrayValue<T>, ArrayOperation<T>>;

#[derive(Clone, Debug, Default)]
pub struct ArrayValue<T>
where
  T: Clone + Debug,
{
  values: Vec<T>,
}

#[derive(Clone, Debug)]
pub enum ArrayOperation<T>
where
  T: Clone + Debug,
{
  GetArgument(GetArgumentOperation<ArrayExpression<T>>),
}
