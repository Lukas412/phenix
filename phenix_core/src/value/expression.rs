use std::fmt::Debug;

use duplicate::duplicate_item;

use crate::{
  evaluate::EvaluateResult, ActionOperation, ActionValue, BooleanOperation, BooleanValue,
  ComplexCreationArguments, Evaluate, NumberOperation, NumberValue, PathOperation,
  PathValue, Runtime, StringOperation, StringValue,
};

#[derive(Clone, Debug)]
pub enum Expression<V, O> {
  Value(V),
  Operation(Box<O>),
}

impl<V, O> Default for Expression<V, O>
where
  V: Default,
{
  fn default() -> Self {
    Self::Value(V::default())
  }
}

impl<V, O> Evaluate<V> for Expression<V, O>
where
  V: Clone,
  O: Evaluate<V>,
{
  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<V> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, arguments),
    }
  }
}

#[duplicate_item(
  ValueType OperationType;
  [ActionValue] [ActionOperation];
  [BooleanValue] [BooleanOperation];
  [NumberValue] [NumberOperation];
  [PathValue] [PathOperation];
  [StringValue] [StringOperation];
)]
impl From<ValueType> for Expression<ValueType, OperationType> {
  fn from(value: ValueType) -> Self {
    Expression::Value(value.into())
  }
}

#[duplicate_item(
  ValueType OperationType;
  [ActionValue] [ActionOperation];
  [BooleanValue] [BooleanOperation];
  [NumberValue] [NumberOperation];
  [PathValue] [PathOperation];
  [StringValue] [StringOperation];
)]
impl From<OperationType> for Expression<ValueType, OperationType> {
  fn from(operation: OperationType) -> Self {
    Expression::Operation(operation.into())
  }
}
