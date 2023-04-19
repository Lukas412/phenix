use derive_more::From;

use crate::{
  evaluate::EvaluateResult,
  operations::{AndOperation, GetArgumentOperation, OrOperation},
  And, AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  HasArgumentOperation, Or, Runtime, ToType,
};

#[derive(Clone, Debug, From)]
pub enum BooleanExpression {
  Value(BooleanValue),
  Operation(BooleanOperation),
}

impl Evaluate for BooleanExpression {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(*value),
      Self::Operation(operation) => operation.evaluate(runtime, arguments),
    }
  }
}

pub type BooleanValue = bool;

impl Evaluate for BooleanValue {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    _runtime: &Runtime,
    _arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    Ok(self.clone())
  }
}

impl TryFrom<AnyValue> for BooleanValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Boolean(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Boolean).into()),
    }
  }
}

impl And for BooleanValue {
  type Output = EvaluateResult<BooleanValue>;

  fn and(self, rhs: Self) -> Self::Output {
    Ok(self && rhs)
  }
}

impl Or for BooleanValue {
  type Output = EvaluateResult<BooleanValue>;

  fn or(self, rhs: Self) -> Self::Output {
    Ok(self || rhs)
  }
}

#[derive(Clone, Debug, From)]
pub enum BooleanOperation {
  And(AndOperation<BooleanExpression>),
  Or(OrOperation<BooleanExpression>),
  HasArgument(HasArgumentOperation),
  GetArgument(GetArgumentOperation<BooleanValue>),
}

impl Evaluate for BooleanOperation {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::And(operation) => operation.evaluate(runtime, arguments),
      Self::Or(operation) => operation.evaluate(runtime, arguments),
      Self::HasArgument(operation) => operation.evaluate(runtime, arguments),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
