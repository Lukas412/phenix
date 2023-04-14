use derive_more::{Display, Error};

use crate::Namespace;

#[derive(Clone, Debug, Display, Error)]
#[display(fmt = "Expression for {namespace} could not be found!")]
pub struct ExpressionNotFoundError {
  namespace: Namespace,
}

impl ExpressionNotFoundError {
  pub fn new(namespace: Namespace) -> Self {
    Self { namespace }
  }
}
