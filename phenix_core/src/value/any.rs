use derive_more::From;
use rust_decimal::Decimal;

use crate::{
  evaluate::EvaluateResult, ActionExpression, ActionOperation, ActionValue, BooleanExpression,
  BooleanValue, CommandValue, Evaluate, EvaluateArguments, GetArgumentOperation, NumberExpression,
  NumberOperation, NumberValue, PathExpression, PathValue, Runtime, TextExpression, TextOperation,
  TextValue,
};

#[derive(Clone, Debug, From)]
pub enum AnyExpression {
  #[from(types(ActionOperation, ActionValue))]
  Action(ActionExpression),
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
  Path(PathExpression),
  #[from(types(TextOperation, TextValue))]
  Text(TextExpression),
}

impl From<&str> for AnyExpression {
  fn from(value: &str) -> Self {
    Self::Text(value.into())
  }
}

impl From<GetArgumentOperation<TextValue>> for AnyExpression {
  fn from(operation: GetArgumentOperation<TextValue>) -> Self {
    Self::Text(operation.into())
  }
}

impl Evaluate for AnyExpression {
  type Result = AnyValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Action(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::Boolean(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::Number(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::Path(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::Text(expression) => expression.evaluate(runtime, arguments).map(Into::into),
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
