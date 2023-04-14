use derive_more::From;
use duplicate::duplicate_item;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  JoinOperation, LinesOperation, Runtime, ToType, WordsOperation,
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

#[duplicate_item(
  OperationType;
  [JoinOperation<TextExpression, TextExpression>];
  [WordsOperation<TextExpression>];
  [LinesOperation<TextExpression>];
  [GetArgumentOperation<TextValue>];
)]
impl From<OperationType> for TextExpression {
  fn from(operation: OperationType) -> Self {
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

impl Evaluate for TextValue {
  type Result = TextValue;

  fn evaluate(
    &self,
    _runtime: &Runtime,
    _arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    Ok(self.clone())
  }
}

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
  Words(WordsOperation<TextExpression>),
  #[from]
  Lines(LinesOperation<TextExpression>),
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
      Self::Join(operation) => operation.evaluate(runtime, arguments),
      Self::Words(operation) => operation.evaluate(runtime, arguments),
      Self::Lines(operation) => operation.evaluate(runtime, arguments),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
