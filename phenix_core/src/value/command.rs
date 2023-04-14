use derive_more::From;

use crate::evaluate::EvaluateResult;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  GetArgumentOperation, Runtime, TextExpression, TextValue, ToType,
};

#[derive(Clone, Debug, From)]
pub enum CommandExpression {
  Value(CommandValue),
  Operation(CommandOperation),
}

impl CommandExpression {
  pub fn new<IntoNameTextExpression, IntoFlagsTextExpression>(
    name: IntoNameTextExpression,
    flags: IntoFlagsTextExpression,
  ) -> Self
  where
    IntoNameTextExpression: Into<TextExpression>,
    IntoFlagsTextExpression: Into<TextExpression>,
  {
    let command = CommandOperation::Expression {
      name: name.into(),
      flags: flags.into(),
    };
    command.into()
  }
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

#[derive(Clone, Debug, From)]
pub struct CommandValue(TextValue);

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
  Expression {
    name: TextExpression,
    flags: TextExpression,
  },
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
      Self::Expression { name, flags } => {
        let name = name.evaluate(runtime, arguments)?;
        let flags = flags.evaluate(runtime, arguments)?;
        let command = name + " " + flags.as_str();
        Ok(command.into())
      }
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
