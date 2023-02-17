use std::num::TryFromIntError;

use derive_more::{Display, Error, From};

use crate::ExpressionNotFoundError;

#[derive(Debug, Display, Error, From)]
pub enum EvaluateErr {
  #[display(fmt = "Error while evaluating creation:\n{error}")]
  ExpressionNotFound {
    #[error(source)]
    error: ExpressionNotFoundError,
  },
  TryFromIntError {
    #[error(source)]
    error: TryFromIntError,
  },
}
