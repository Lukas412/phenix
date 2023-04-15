use derive_more::{Display, Error, From};

use crate::Identifier;

#[derive(Clone, Debug, Display, Error, From)]
pub enum GetArgumentOperationError {
  #[from(forward)]
  ArgumentNotFound {
    #[error(source)]
    error: ArgumentNotFoundError
  },
}

#[derive(Clone, Debug, Display, Error)]
#[display(fmt = "Argument was not found: {identifier}")]
pub struct ArgumentNotFoundError {
  identifier: Identifier,
}

impl ArgumentNotFoundError {
  pub fn new(identifier: Identifier) -> Self {
    Self { identifier }
  }
}
