use std::ops::{Add, Sub};

use derive_more::{Display, From};
use rust_decimal::Decimal;

use crate::error::ExtractTypeFromAnyError;
use crate::evaluate::EvaluateResult;
use crate::operations::{
  AddOperation, EqualsOperation, GetArgumentOperation, SubOperation, ToBooleanOperation,
};
use crate::{AnyValue, ContextExt, Evaluate, EvaluateError, Runtime, ToType};

#[derive(Clone, Debug, From)]
pub enum NumberExpression {
  #[from(types(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Decimal))]
  Value(NumberValue),
  #[from]
  Operation(NumberOperation),
}

impl From<AddOperation<NumberExpression>> for NumberExpression {
  fn from(operation: AddOperation<NumberExpression>) -> Self {
    Self::Operation(operation.into())
  }
}

impl From<GetArgumentOperation<NumberValue>> for NumberExpression {
  fn from(operation: GetArgumentOperation<NumberValue>) -> Self {
    Self::Operation(operation.into())
  }
}

impl<Context> Evaluate<Context> for NumberExpression
where
  Context: ContextExt,
{
  type Result = NumberValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, context),
    }
  }
}

#[derive(Clone, Debug, Display, PartialEq, Eq, From)]
#[from(forward)]
pub struct NumberValue(Decimal);

impl Add for NumberValue {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl Sub for NumberValue {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}

impl TryFrom<AnyValue> for NumberValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Number(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Number).into()),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum NumberOperation {
  #[from]
  Add(AddOperation<NumberExpression>),
  #[from]
  Sub(SubOperation<NumberExpression>),
  #[from]
  Equals(EqualsOperation<NumberExpression>),
  #[from]
  ToBoolean(ToBooleanOperation<NumberExpression>),
  #[from]
  GetArgument(GetArgumentOperation<NumberValue>),
}

impl<Context> Evaluate<Context> for NumberOperation
where
  Context: ContextExt,
{
  type Result = NumberValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Add(operation) => operation.evaluate(runtime, context),
      Self::Sub(operation) => operation.evaluate(runtime, context),
      Self::Equals(_) => todo!(),
      Self::ToBoolean(_) => todo!(),
      Self::GetArgument(operation) => operation.evaluate(runtime, context),
    }
  }
}
