use std::path::PathBuf;

use derive_more::From;
use duplicate::duplicate_item;

use crate::evaluate::EvaluateResult;
use crate::{
  AnyValue, ContextExt, Evaluate, EvaluateError, ExtractTypeFromAnyError, GetArgumentOperation,
  PathJoinOperation, Runtime, TextExpression, ToPathOperation, ToType,
};

#[derive(Clone, Debug, From)]
pub enum PathExpression<Context> {
  #[from]
  Value(PathValue),
  #[from(types(PathJoinOperation<Context>))]
  Operation(PathOperation<Context>),
}

#[duplicate_item(
  OperationType;
  [ToPathOperation<TextExpression<Context>>];
  [GetArgumentOperation<PathValue>];
)]
impl<Context> From<OperationType> for PathExpression<Context> {
  fn from(operation: OperationType) -> Self {
    Self::Operation(operation.into())
  }
}

impl<Context> Evaluate<Context> for PathExpression<Context>
where
  Context: ContextExt,
{
  type Result = PathValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, context),
    }
  }
}

pub type PathValue = PathBuf;

impl TryFrom<AnyValue> for PathValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Path(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Action).into()),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum PathOperation<Context> {
  StringToPath(ToPathOperation<TextExpression<Context>>),
  Join(PathJoinOperation<Context>),
  GetArgument(GetArgumentOperation<PathValue>),
}

impl<Context> Evaluate<Context> for PathOperation<Context>
where
  Context: ContextExt,
{
  type Result = PathValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    match self {
      Self::StringToPath(operation) => operation.evaluate(runtime, context),
      Self::Join(operation) => operation.evaluate(runtime, context),
      Self::GetArgument(operation) => operation.evaluate(runtime, context),
    }
  }
}
