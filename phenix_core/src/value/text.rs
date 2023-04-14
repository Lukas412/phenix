use derive_more::From;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  JoinOperation, Runtime, ToType,
};

#[derive(Clone, Debug, From)]
pub enum TextExpression {
  #[from]
  Value(TextValue),
  #[from]
  Operation(TextOperation),
}

impl From<&str> for TextExpression {
  fn from(value: &str) -> Self {
    Self::Value(value.into())
  }
}

impl From<Vec<TextExpression>> for TextExpression {
  fn from(expressions: Vec<TextExpression>) -> Self {
    JoinOperation::new("", expressions).into()
  }
}

impl From<JoinOperation<TextExpression, TextExpression>> for TextExpression {
  fn from(operation: JoinOperation<TextExpression, TextExpression>) -> Self {
    Self::Operation(operation.into())
  }
}

impl From<GetArgumentOperation<TextValue>> for TextExpression {
  fn from(operation: GetArgumentOperation<TextValue>) -> Self {
    Self::Operation(operation.into())
  }
}

impl Evaluate for TextExpression {
  type Result = TextValue;

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

pub type TextValue = String;

impl TryFrom<AnyValue> for TextValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::String(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Action).into()),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum TextOperation {
  #[from]
  Join(JoinOperation<TextExpression, TextExpression>),
  #[from]
  GetArgument(GetArgumentOperation<TextValue>),
}

impl Evaluate for TextOperation {
  type Result = TextValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Join(operation) => todo!(),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
