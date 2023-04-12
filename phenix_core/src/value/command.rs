use crate::evaluate::EvaluateResult;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  GetArgumentOperation, Runtime, TextExpression, ToType,
};
use derive_more::From;

use super::array::ArrayExpression;

#[derive(Clone, Debug, From)]
pub enum CommandExpression {
  Value(CommandValue),
  Operation(CommandOperation),
}

impl Evaluate for CommandExpression {
  type Result = CommandValue;

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

#[derive(Clone, Debug)]
pub struct CommandValue {
  name: TextExpression,
  arguments: ArrayExpression<TextExpression>,
}

impl CommandValue {
  pub fn new<IntoTextExpression, IntoTextArrayExpression>(
    name: IntoTextExpression,
    arguments: IntoTextArrayExpression,
  ) -> Self
  where
    IntoTextExpression: Into<TextExpression>,
    IntoTextArrayExpression: Into<ArrayExpression<TextExpression>>,
  {
    Self {
      name: name.into(),
      arguments: arguments.into(),
    }
  }
}

impl TryFrom<AnyValue> for CommandValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Command(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Command).into()),
    }
  }
}

#[derive(Clone, Debug)]
pub enum CommandOperation {
  GetArgument(GetArgumentOperation<CommandValue>),
}

impl Evaluate for CommandOperation {
  type Result = CommandValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
