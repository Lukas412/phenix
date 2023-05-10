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
pub enum CommandOperation<Context> {
  Expression(TextExpression<Context>),
  GetArgument(GetArgumentOperation<CommandValue>),
}

impl<Context> CommandOperation<Context> {
  pub fn new<IntoTextExpression>(expression: IntoTextExpression) -> Self
  where
    IntoTextExpression: Into<TextExpression<Context>>,
  {
    Self::Expression(expression.into())
  }
}

impl<Into1, Into2, Context> From<(Into1, Into2)> for CommandOperation<Context>
where
  Into1: Into<TextExpression<Context>>,
  Into2: Into<TextExpression<Context>>,
{
  fn from(values: (Into1, Into2)) -> Self {
    Self::new(TextWordsOperation::from(values))
  }
}

impl<Into1, Into2, Into3, Context> From<(Into1, Into2, Into3)> for CommandOperation<Context>
where
  Into1: Into<TextExpression<Context>>,
  Into2: Into<TextExpression<Context>>,
  Into3: Into<TextExpression<Context>>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::new(TextWordsOperation::from(values))
  }
}

impl<Into1, Into2, Into3, Into4, Context> From<(Into1, Into2, Into3, Into4)>
  for CommandOperation<Context>
where
  Into1: Into<TextExpression<Context>>,
  Into2: Into<TextExpression<Context>>,
  Into3: Into<TextExpression<Context>>,
  Into4: Into<TextExpression<Context>>,
{
  fn from(values: (Into1, Into2, Into3, Into4)) -> Self {
    Self::new(TextWordsOperation::from(values))
  }
}

impl<Context> Evaluate<Context> for CommandOperation<Context> {
  type Result = CommandValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Expression(expression) => expression.evaluate(runtime, context).map(CommandValue::new),
      Self::GetArgument(operation) => operation.evaluate(runtime, context),
    }
  }
}
