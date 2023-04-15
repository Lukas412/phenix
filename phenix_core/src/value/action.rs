use derive_more::From;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::array::ArrayValue;
use crate::value::command::CommandExpression;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  PathExpression, Runtime, TextExpression, ToType,
};

#[derive(Clone, Debug, From)]
pub enum ActionExpression {
  #[from]
  Value(ActionValue),
  #[from]
  Operation(ActionOperation),
}

impl Evaluate for ActionExpression {
  type Result = ActionValue;

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

#[derive(Clone, Debug, From)]
pub enum ActionValue {
  ChangeLocation {
    location: PathExpression,
    actions: ArrayValue<ActionExpression>,
  },
  #[from(forward)]
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
  GetArgument(GetArgumentOperation<ActionValue>),
}

impl Evaluate for ActionOperation {
  type Result = ActionValue;

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
