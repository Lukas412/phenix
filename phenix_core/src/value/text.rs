use derive_more::From;
use duplicate::duplicate_item;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::{
  AnyValue, ConditionOperation, ContextSwitchOperation, DynamicContext, Evaluate, EvaluateError,
  ExtractTypeFromAnyError, Runtime, TextBlockOperation, TextJoinOperation, TextLinesOperation,
  TextWordsOperation, ToType,
};

#[derive(Clone, Debug, From)]
pub enum TextExpression<Context> {
  #[from]
  Value(TextValue),
  #[from(types(TextBlockOperation, TextWordsOperation))]
  Operation(TextOperation<Context>),
}

impl<Context> From<&str> for TextExpression<Context> {
  fn from(value: &str) -> Self {
    Self::Value(value.into())
  }
}

impl<Context> From<Vec<TextExpression<Context>>> for TextExpression<Context> {
  fn from(expressions: Vec<TextExpression<Context>>) -> Self {
    TextBlockOperation::new(expressions).into()
  }
}

#[duplicate_item(
  OperationType;
  [TextJoinOperation<TextExpression<Context>, TextExpression<Context>>];
  [TextLinesOperation<TextExpression<Context>>];
  [ConditionOperation<TextExpression<Context>>];
  [GetArgumentOperation<TextValue>];
)]
impl<Context> From<OperationType> for TextExpression<Context> {
  fn from(operation: OperationType) -> Self {
    Self::Operation(operation.into())
  }
}

impl<Context> Evaluate<Context> for TextExpression<Context> {
  type Result = TextValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &DynamicContext,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, arguments),
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
  Block(TextBlockOperation),
  Words(TextWordsOperation),
  Lines(TextLinesOperation<TextExpression<Context>>),
  Condition(ConditionOperation<TextExpression<Context>>),
  GetArgument(GetArgumentOperation<TextValue>),
  ContextSwitch(ContextSwitchOperation<TextExpression<Context>, Context>),
}

impl<Context> Evaluate<Context> for TextOperation<Context> {
  type Result = TextValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &DynamicContext,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Join(operation) => operation.evaluate(runtime, arguments),
      Self::Block(operation) => operation.evaluate(runtime, arguments),
      Self::Words(operation) => operation.evaluate(runtime, arguments),
      Self::Lines(operation) => operation.evaluate(runtime, arguments),
      Self::Condition(operation) => operation.evaluate(runtime, arguments),
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
      Self::ContextSwitch(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
