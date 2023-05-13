use derive_more::From;
use duplicate::duplicate_item;
use rust_decimal::Decimal;

use crate::{
  evaluate::EvaluateResult, ActionExpression, ActionValue, BooleanExpression, BooleanValue,
  CommandValue, ContextExt, Evaluate, GetArgumentOperation, NumberExpression, NumberOperation,
  NumberValue, PathExpression, PathValue, Runtime, TextExpression, TextOperation, TextValue,
};

#[derive(Clone, Debug, From)]
pub enum AnyExpression<Context> {
  #[from(types(ActionValue))]
  Action(ActionExpression<Context>),
  #[from]
  Boolean(BooleanExpression),
  #[from(types(
    NumberOperation,
    NumberValue,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    Decimal
  ))]
  Number(NumberExpression),
  #[from]
  Path(PathExpression<Context>),
  #[from(types(TextValue))]
  Text(TextExpression<Context>),
}

impl<Context> From<&str> for AnyExpression<Context> {
  fn from(value: &str) -> Self {
    Self::Text(value.into())
  }
}

#[duplicate_item(
  Operation;
  [GetArgumentOperation<TextValue>];
  [TextOperation<Context>];
)]
impl<Context> From<Operation> for AnyExpression<Context> {
  fn from(operation: Operation) -> Self {
    Self::Text(operation.into())
  }
}

impl<Context> Evaluate<Context> for AnyExpression<Context>
where
  Context: ContextExt,
{
  type Result = AnyValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Action(expression) => expression.evaluate(runtime, context).map(Into::into),
      Self::Boolean(expression) => expression.evaluate(runtime, context).map(Into::into),
      Self::Number(expression) => expression.evaluate(runtime, context).map(Into::into),
      Self::Path(expression) => expression.evaluate(runtime, context).map(Into::into),
      Self::Text(expression) => expression.evaluate(runtime, context).map(Into::into),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum AnyValue {
  Action(ActionValue),
  Boolean(BooleanValue),
  Command(CommandValue),
  Number(NumberValue),
  Path(PathValue),
  String(TextValue),
}
