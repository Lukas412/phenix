use derive_more::From;
use duplicate::duplicate_item;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError, Runtime,
  TextBlockOperation, TextJoinOperation, TextLinesOperation, TextWordsOperation, ToType,
};

#[derive(Clone, Debug, From)]
pub enum TextExpression {
  #[from]
  Value(TextValue),
  #[from(types(TextBlockOperation, TextWordsOperation))]
  Operation(TextOperation),
}

impl From<&str> for TextExpression {
  fn from(value: &str) -> Self {
    Self::Value(value.into())
  }
}

impl From<Vec<TextExpression>> for TextExpression {
  fn from(expressions: Vec<TextExpression>) -> Self {
    TextJoinOperation::new("", expressions).into()
  }
}

#[duplicate_item(
  OperationType;
  [TextJoinOperation<TextExpression, TextExpression>];
  [TextLinesOperation<TextExpression>];
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
  Join(TextJoinOperation<TextExpression, TextExpression>),
  #[from]
  Block(TextBlockOperation),
  #[from]
  Words(TextWordsOperation),
  #[from]
  Lines(TextLinesOperation<TextExpression>),
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
      Self::Block(operation) => operation.evaluate(runtime, arguments),
      Self::Words(operation) => operation.evaluate(runtime, arguments),
      Self::Lines(operation) => operation.evaluate(runtime, arguments),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
