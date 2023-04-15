use derive_more::{Display, From};

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError, Runtime,
  ToType,
};

#[derive(Clone, Debug, From)]
pub enum TextExpression {
  #[from(types(String))]
  Value(TextValue),
  #[from]
  Operation(TextOperation),
}

impl From<&str> for TextExpression {
  fn from(value: &str) -> Self {
    Self::Value(value.into())
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
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, arguments),
    }
  }
}

#[derive(Clone, Debug, Default, Display, PartialEq, Eq, From)]
#[display(fmt = "\"{value}\"")]
pub struct TextValue {
  value: String,
}

impl From<&str> for TextValue {
  fn from(value: &str) -> Self {
    Self {
      value: value.into(),
    }
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
  GetArgument(GetArgumentOperation<TextValue>),
}

impl Evaluate for TextOperation {
  type Result = TextValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
