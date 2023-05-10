use derive_more::From;

use crate::evaluate::EvaluateResult;
use crate::{
  AnyValue, AsBash, DynamicContext, Evaluate, EvaluateError, ExtractTypeFromAnyError,
  GetArgumentOperation, Runtime, TextExpression, TextValue, TextWordsOperation, ToType,
};

#[derive(Clone, Debug, From)]
pub struct CommandValue(TextValue);

impl CommandValue {
  pub fn new<IntoTextValue>(value: IntoTextValue) -> Self
  where
    IntoTextValue: Into<TextValue>,
  {
    Self(value.into())
  }
}

impl AsBash for CommandValue {
  fn as_bash(&self) -> String {
    self.0.clone()
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
  Expression(TextExpression),
  GetArgument(GetArgumentOperation<CommandValue>),
}

impl CommandOperation {
  pub fn new<IntoTextExpression>(expression: IntoTextExpression) -> Self
  where
    IntoTextExpression: Into<TextExpression>,
  {
    Self::Expression(expression.into())
  }
}

impl<Into1, Into2> From<(Into1, Into2)> for CommandOperation
where
  Into1: Into<TextExpression>,
  Into2: Into<TextExpression>,
{
  fn from(values: (Into1, Into2)) -> Self {
    Self::new(TextWordsOperation::from(values))
  }
}

impl<Into1, Into2, Into3> From<(Into1, Into2, Into3)> for CommandOperation
where
  Into1: Into<TextExpression>,
  Into2: Into<TextExpression>,
  Into3: Into<TextExpression>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::new(TextWordsOperation::from(values))
  }
}

impl<Into1, Into2, Into3, Into4> From<(Into1, Into2, Into3, Into4)> for CommandOperation
where
  Into1: Into<TextExpression>,
  Into2: Into<TextExpression>,
  Into3: Into<TextExpression>,
  Into4: Into<TextExpression>,
{
  fn from(values: (Into1, Into2, Into3, Into4)) -> Self {
    Self::new(TextWordsOperation::from(values))
  }
}

impl Evaluate for CommandOperation {
  type Result = CommandValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &DynamicContext,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Expression(expression) => expression
        .evaluate(runtime, arguments)
        .map(CommandValue::new),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
