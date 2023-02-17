use std::ops::Add;

use crate::evaluate::EvaluateResult;
use crate::operations::{
  AddOperation, EqualsOperation, EvaluateAdd, EvaluateEquals, EvaluateSub, GetArgumentOperation,
  SubOperation,
};
use crate::value::expression::Expression;
use crate::{runtime, BooleanValue, ComplexCreationArguments, Evaluate, EvaluateErr, Runtime};
use derive_more::{Add, Display, From};
use duplicate::duplicate_item;
use rust_decimal::Decimal;

pub type NumberExpression = Expression<NumberValue, NumberOperation>;

#[duplicate_item(FromType; [i8]; [i16]; [i32]; [i64]; [u8]; [u16]; [u32]; [u64]; [Decimal];)]
impl From<FromType> for NumberExpression {
  fn from(value: FromType) -> Self {
    Self::Value(value.into())
  }
}

#[duplicate_item(FromType; [AddOperation<NumberExpression>]; [GetArgumentOperation<NumberExpression>];)]
impl From<FromType> for NumberExpression {
  fn from(operation: FromType) -> Self {
    Self::Operation(Box::new(operation.into()))
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

#[derive(Clone, Debug, From)]
pub enum NumberOperation {
  Add(AddOperation<NumberExpression>),
  Sub(SubOperation<NumberExpression>),
  Equals(EqualsOperation<NumberExpression>),
  GetArgument(GetArgumentOperation<NumberExpression>),
}

impl Evaluate<NumberValue> for NumberOperation {
  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<NumberValue> {
    match self {
      Self::Add(operation) => operation.evaluate(runtime, arguments),
      Self::Sub(_) => todo!(),
      Self::Equals(_) => todo!(),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
