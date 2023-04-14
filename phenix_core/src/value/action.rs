use derive_more::From;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::command::CommandExpression;
use crate::{
  AnyValue, CommandOperation, CommandValue, ComplexCreationArguments, Evaluate, EvaluateError,
  ExtractTypeFromAnyError, PathExpression, Runtime, TextExpression, ToType,
};

#[derive(Clone, Debug, From)]
pub enum ActionExpression {
  #[from(types(CommandExpression, CommandValue, CommandOperation))]
  Value(ActionValue),
  #[from]
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
  ChangeLocation {
    location: PathExpression,
    actions: Box<ActionExpression>,
  },
  #[from(types(CommandValue, CommandOperation))]
  ExecuteCommand(CommandExpression),
  WriteContent {
    file: PathExpression,
    content: TextExpression,
  },
  EnsureDirectory {
    file: PathExpression,
  },
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
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
