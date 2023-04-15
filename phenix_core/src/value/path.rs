use std::path::PathBuf;

use derive_more::From;
use duplicate::duplicate_item;

use crate::evaluate::EvaluateResult;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  GetArgumentOperation, PathJoinOperation, Runtime, TextExpression, ToPathOperation, ToType,
};

#[derive(Clone, Debug, From)]
pub enum PathExpression {
  #[from]
  Value(PathValue),
  #[from(types(PathJoinOperation))]
  Operation(PathOperation),
}

#[duplicate_item(
  OperationType;
  [ToPathOperation<TextExpression>];
  [GetArgumentOperation<PathValue>];
)]
impl From<OperationType> for PathExpression {
  fn from(operation: OperationType) -> Self {
    Self::Operation(operation.into())
  }
}

impl Evaluate for PathExpression {
  type Result = PathValue;

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

pub type PathValue = PathBuf;

impl TryFrom<AnyValue> for PathValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Path(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Action).into()),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum PathOperation {
  StringToPath(ToPathOperation<TextExpression>),
  Join(PathJoinOperation),
  GetArgument(GetArgumentOperation<PathValue>),
}

impl Evaluate for PathOperation {
  type Result = PathValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::StringToPath(operation) => operation.evaluate(runtime, arguments),
      Self::Join(operation) => operation.evaluate(runtime, arguments),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
