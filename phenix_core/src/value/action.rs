use derive_more::From;
use duplicate::duplicate_item;
use itertools::Itertools;

pub use {
  command::{CommandOperation, CommandValue},
  location::{LocationOperation, LocationValue},
};

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, AsBash, ContextExt, ContextSwitchOperation, Evaluate, EvaluateError,
  ExtractTypeFromAnyError, PathValue, Runtime, TextValue, ToType,
};

mod command;
mod location;

#[derive(Clone, Debug, From)]
pub enum ActionExpression<Context> {
  #[from(types(CommandValue))]
  Value(ActionValue),
  #[from]
  Operation(ActionOperation<Context>),
}

impl<Context> From<Vec<ActionValue>> for ActionExpression<Context> {
  fn from(values: Vec<ActionValue>) -> Self {
    Self::Value(values.into())
  }
}

#[duplicate_item(
OperationType;
[Vec<ActionExpression<Context>>];
[CommandOperation<Context>];
[LocationOperation<Context>];
[ContextSwitchOperation<ActionExpression<Context>, Context>];
)]
impl<Context> From<OperationType> for ActionExpression<Context> {
  fn from(expressions: OperationType) -> Self {
    Self::Operation(expressions.into())
  }
}

impl<Into1, Into2, Into3, Context> From<(Into1, Into2, Into3)> for ActionExpression<Context>
where
  Into1: Into<ActionExpression<Context>>,
  Into2: Into<ActionExpression<Context>>,
  Into3: Into<ActionExpression<Context>>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::Operation(values.into())
  }
}

impl<Context> Evaluate<Context> for ActionExpression<Context>
where
  Context: ContextExt,
{
  type Result = ActionValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, context),
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
    file: PathValue,
  },
}

impl AsBash for ActionValue {
  fn as_bash(&self) -> String {
    match self {
      Self::Array(values) => values.iter().map(AsBash::as_bash).join("\n"),
      Self::Location(location) => location.as_bash(),
      Self::Command(command) => command.as_bash(),
      Self::WriteContent { .. } => todo!(),
      Self::EnsureDirectory { .. } => todo!(),
    }
  }
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

impl<Into1, Into2, Into3> From<(Into1, Into2, Into3)> for ActionValue
where
  Into1: Into<ActionValue>,
  Into2: Into<ActionValue>,
  Into3: Into<ActionValue>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::Array(vec![values.0.into(), values.1.into(), values.2.into()])
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
pub enum ActionOperation<Context> {
  Array(Vec<ActionExpression<Context>>),
  Command(CommandOperation<Context>),
  Location(LocationOperation<Context>),
  GetArgument(GetArgumentOperation<ActionValue>),
  ContextSwitch(ContextSwitchOperation<ActionExpression<Context>, Context>),
}

impl<Into1, Into2, Context> From<(Into1, Into2)> for ActionOperation<Context>
where
  Into1: Into<ActionExpression<Context>>,
  Into2: Into<ActionExpression<Context>>,
{
  fn from(values: (Into1, Into2)) -> Self {
    Self::from(vec![values.0.into(), values.1.into()])
  }
}

impl<Into1, Into2, Into3, Context> From<(Into1, Into2, Into3)> for ActionOperation<Context>
where
  Into1: Into<ActionExpression<Context>>,
  Into2: Into<ActionExpression<Context>>,
  Into3: Into<ActionExpression<Context>>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::from(vec![values.0.into(), values.1.into(), values.2.into()])
  }
}

impl<Context> Evaluate<Context> for ActionOperation<Context>
where
  Context: ContextExt,
{
  type Result = ActionValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Array(expressions) => expressions
        .iter()
        .map(|expression| expression.evaluate(runtime, context))
        .collect::<EvaluateResult<Vec<_>>>()
        .map(Into::into),
      Self::Command(operation) => operation.evaluate(runtime, context).map(Into::into),
      Self::Location(operation) => operation.evaluate(runtime, context).map(Into::into),
      Self::GetArgument(operation) => operation.evaluate(runtime, context),
      Self::ContextSwitch(operation) => operation.evaluate(runtime, context),
    }
  }
}
