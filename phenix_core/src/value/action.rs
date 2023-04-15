use derive_more::From;

pub use {
  command::{CommandOperation, CommandValue},
  location::{LocationOperation, LocationValue},
};

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  PathExpression, PathValue, Runtime, TextValue, ToType,
};

mod command;
mod location;

#[derive(Clone, Debug, From)]
pub enum ActionExpression {
  #[from(types(CommandValue))]
  Value(ActionValue),
  #[from(types(CommandOperation, LocationOperation))]
  Operation(ActionOperation),
}

impl From<Vec<ActionValue>> for ActionExpression {
  fn from(values: Vec<ActionValue>) -> Self {
    Self::Value(values.into())
  }
}

impl From<Vec<ActionExpression>> for ActionExpression {
  fn from(expressions: Vec<ActionExpression>) -> Self {
    Self::Operation(expressions.into())
  }
}

impl Evaluate for ActionExpression {
  type Result = ActionValue;

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

#[derive(Clone, Debug, From)]
pub enum ActionValue {
  #[from]
  Array(Vec<ActionValue>),
  #[from]
  Location(LocationValue),
  #[from]
  Command(CommandValue),
  WriteContent {
    file: PathValue,
    content: TextValue,
  },
  EnsureDirectory {
    file: PathExpression,
  },
}

impl<Into1, Into2> From<(Into1, Into2)> for ActionValue
where
  Into1: Into<ActionValue>,
  Into2: Into<ActionValue>,
{
  fn from(values: (Into1, Into2)) -> Self {
    Self::Array(vec![values.0.into(), values.1.into()])
  }
}

impl TryFrom<AnyValue> for ActionValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Action(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Action).into()),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum ActionOperation {
  #[from]
  Array(Vec<ActionExpression>),
  #[from]
  Command(CommandOperation),
  #[from]
  Location(LocationOperation),
  #[from]
  GetArgument(GetArgumentOperation<ActionValue>),
}

impl Evaluate for ActionOperation {
  type Result = ActionValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Array(expressions) => expressions
        .iter()
        .map(|expression| expression.evaluate(runtime, arguments))
        .collect::<EvaluateResult<Vec<_>>>()
        .map(Into::into),
      Self::Command(operation) => operation.evaluate(runtime, arguments).map(Into::into),
      Self::Location(operation) => operation.evaluate(runtime, arguments).map(Into::into),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
