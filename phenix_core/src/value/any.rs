use std::ops::Add;

use derive_more::{Display, From};
use duplicate::duplicate_item;

use crate::{
  evaluate::EvaluateResult, ActionExpression, ActionValue, BooleanExpression, BooleanValue,
  ComplexCreationArguments, Evaluate, EvaluateError, NumberExpression, NumberValue, PathExpression,
  PathValue, Runtime, StringExpression, StringValue,
};

use super::expression;

#[derive(Clone, Debug, From)]
pub enum AnyExpression {
  #[from]
  Action(ActionExpression),
  #[from]
  Boolean(BooleanExpression),
  #[from(forward)]
  Number(NumberExpression),
  #[from]
  Path(PathExpression),
  #[from]
  String(StringExpression),
}

impl Evaluate<AnyValue> for AnyExpression {
  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<AnyValue> {
    match self {
      Self::Action(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::Boolean(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::Number(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::Path(expression) => expression.evaluate(runtime, arguments).map(Into::into),
      Self::String(expression) => expression.evaluate(runtime, arguments).map(Into::into),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum AnyValue {
  Action(ActionValue),
  Boolean(BooleanValue),
  Number(NumberValue),
  Path(PathValue),
  String(StringValue),
}

impl Add for AnyValue {
  type Output = AnyValue;

  fn add(self, rhs: Self) -> Self::Output {
    todo!("Add for AnyValue not implemented")
  }
}