use derive_more::{From};

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::expression::Expression;
use crate::{AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError, Runtime, ToType};

use std::fmt::Display;
use std::path::{PathBuf};

pub type PathExpression = Expression<PathValue, PathOperation>;

#[derive(Clone, Debug, PartialEq, Eq, From)]
#[from(forward)]
pub struct PathValue(PathBuf);

impl Display for PathValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.to_string_lossy())
  }
}

impl TryFrom<AnyValue> for PathValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::Path(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Action).into()),
    }
  }
}

#[derive(Clone, Debug)]
pub enum PathOperation {
  GetArgument(GetArgumentOperation<PathValue>),
}

impl Evaluate for PathOperation {
  type Result = PathValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
