use derive_more::From;

use crate::{
  evaluate::EvaluateResult, AndOperation, AnyValue, ContextExt, Evaluate, EvaluateError,
  ExtractTypeFromAnyError, GetArgumentOperation, HasArgumentOperation, OrOperation, Runtime,
  ToType,
};

#[derive(Clone, Debug, From)]
pub enum BooleanExpression {
  #[from]
  Value(BooleanValue),
  #[from(types(HasArgumentOperation, AndOperation, OrOperation))]
  Operation(BooleanOperation),
}

impl From<GetArgumentOperation<BooleanValue>> for BooleanExpression {
  fn from(operation: GetArgumentOperation<BooleanValue>) -> Self {
    Self::Operation(operation.into())
  }
}

impl<Context> Evaluate<Context> for BooleanExpression
where
  Context: ContextExt,
{
  type Result = BooleanValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(*value),
      Self::Operation(operation) => operation.evaluate(runtime, context),
    }
  }
}

pub type BooleanValue = bool;

impl<Context> Evaluate<Context> for BooleanValue {
  type Result = BooleanValue;

  fn evaluate(&self, _runtime: &Runtime, _context: &Context) -> EvaluateResult<Self::Result> {
    Ok(self.clone())
  }
}

impl TryFrom<AnyValue> for BooleanValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Boolean(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Boolean).into()),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum BooleanOperation {
  And(AndOperation),
  Or(OrOperation),
  HasArgument(HasArgumentOperation),
  GetArgument(GetArgumentOperation<BooleanValue>),
}

impl<Context> Evaluate<Context> for BooleanOperation
where
  Context: ContextExt,
{
  type Result = BooleanValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::And(operation) => operation.evaluate(runtime, context),
      Self::Or(operation) => operation.evaluate(runtime, context),
      Self::HasArgument(operation) => operation.evaluate(runtime, context),
      Self::GetArgument(operation) => operation.evaluate(runtime, context),
    }
  }
}
