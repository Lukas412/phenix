use std::num::TryFromIntError;

use derive_more::{Display, Error, From};

use crate::{ExpressionNotFoundError, GetArgumentOperationError};
use crate::error::ExtractTypeFromAnyError;

#[derive(Debug, Display, Error, From)]
pub enum EvaluateError {
  #[display(fmt = "Error while evaluating:\n{error}")]
  #[from(forward)]
  GetArgument {
    #[error(source)]
    error: GetArgumentOperationError,
  },
  #[display(fmt = "Error while evaluating:\n{error}")]
  #[from]
  ExpressionNotFound {
    #[error(source)]
    error: ExpressionNotFoundError,
  },
  #[display(fmt = "Error while evaluating:\n{error}")]
  #[from]
  ExtractTypeFromAny {
    #[error(source)]
    error: ExtractTypeFromAnyError,
  },
  #[display(fmt = "Error while evaluating:\n{error}")]
  #[from]
  TryFromIntError {
    #[error(source)]
    error: TryFromIntError,
  },
}
