use derive_more::From;
use duplicate::duplicate_item;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, ConditionOperation, ContextExt, ContextSwitchOperation, Evaluate, EvaluateError,
  ExtractTypeFromAnyError, Runtime, TextBlockOperation, TextJoinOperation, TextLinesOperation,
  TextWordsOperation, ToType,
};

#[derive(Clone, Debug, From)]
pub enum TextExpression<Context> {
  Value(TextValue),
  Operation(TextOperation<Context>),
}

impl<Context> From<&str> for TextExpression<Context> {
  fn from(value: &str) -> Self {
    Self::Value(value.into())
  }
}

#[duplicate_item(
  OperationType;
  [TextJoinOperation<TextExpression<Context>, TextExpression<Context>>];
  [TextBlockOperation<Context>];
  [TextWordsOperation<Context>];
  [TextLinesOperation<Context>];
  [ConditionOperation<TextExpression<Context>>];
  [GetArgumentOperation<TextValue>];
)]
impl<Context> From<OperationType> for TextExpression<Context> {
  fn from(operation: OperationType) -> Self {
    Self::Operation(operation.into())
  }
}

impl<Context> From<Vec<TextExpression<Context>>> for TextExpression<Context> {
  fn from(expressions: Vec<TextExpression<Context>>) -> Self {
    TextBlockOperation::new(expressions).into()
  }
}

impl<Context> Evaluate<Context> for TextExpression<Context>
where
  Context: ContextExt,
{
  type Result = TextValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, context),
    }
  }
}

pub type TextValue = String;

impl<Context> Evaluate<Context> for TextValue {
  type Result = TextValue;

  fn evaluate(&self, _runtime: &Runtime, _context: &Context) -> EvaluateResult<Self::Result> {
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
pub enum TextOperation<Context> {
  Join(TextJoinOperation<TextExpression<Context>, TextExpression<Context>>),
  Block(TextBlockOperation<Context>),
  Words(TextWordsOperation<Context>),
  Lines(TextLinesOperation<Context>),
  Condition(ConditionOperation<TextExpression<Context>>),
  GetArgument(GetArgumentOperation<TextValue>),
  ContextSwitch(ContextSwitchOperation<TextExpression<Context>, Context>),
}

impl<Context> Evaluate<Context> for TextOperation<Context>
where
  Context: ContextExt,
{
  type Result = TextValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Join(operation) => operation.evaluate(runtime, context),
      Self::Block(operation) => operation.evaluate(runtime, context),
      Self::Words(operation) => operation.evaluate(runtime, context),
      Self::Lines(operation) => operation.evaluate(runtime, context),
      Self::Condition(operation) => operation.evaluate(runtime, context),
      Self::GetArgument(operation) => operation.evaluate(runtime, context),
      Self::ContextSwitch(operation) => operation.evaluate(runtime, context),
    }
  }
}
