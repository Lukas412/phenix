use std::ops::{Add, Sub};

use derive_more::{Display, From};
use rust_decimal::Decimal;

use crate::error::ExtractTypeFromAnyError;
use crate::evaluate::EvaluateResult;
use crate::operations::{
  AddOperation, EqualsOperation, GetArgumentOperation, SubOperation, ToBooleanOperation,
};
use crate::{AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, Runtime, ToType};

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

impl Evaluate for NumberExpression {
  type Result = NumberValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, arguments),
    }
  }
}

#[derive(Clone, Debug, Display, PartialEq, Eq, From)]
#[from(forward)]
pub struct NumberValue(Decimal);

impl Add for NumberValue {
  type Output = EvaluateResult<Self>;

  fn add(self, rhs: Self) -> Self::Output {
    let result = self.0 + rhs.0;
    Ok(result.into())
  }
}

impl Sub for NumberValue {
  type Output = EvaluateResult<Self>;

  fn sub(self, rhs: Self) -> Self::Output {
    let result = self.0 - rhs.0;
    Ok(result.into())
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
  #[from(forward)]
  GetArgument(GetArgumentOperation<NumberValue>),
}

impl Evaluate for NumberOperation {
  type Result = NumberValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Add(operation) => operation.evaluate(runtime, arguments),
      Self::Sub(operation) => operation.evaluate(runtime, arguments),
      Self::Equals(_) => todo!(),
      Self::ToBoolean(_) => todo!(),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
