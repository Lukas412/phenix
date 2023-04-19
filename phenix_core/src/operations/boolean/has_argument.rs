use crate::evaluate::EvaluateResult;
use crate::{BooleanValue, Evaluate, EvaluateArguments, Identifier, Runtime};

#[derive(Clone, Debug)]
pub struct HasArgumentOperation {
  identifier: Identifier,
}

impl HasArgumentOperation {
  pub fn new<IntoIdentifier>(identifier: IntoIdentifier) -> Self
  where
    IntoIdentifier: Into<Identifier>,
  {
    Self {
      identifier: identifier.into(),
    }
  }
}

impl Evaluate for HasArgumentOperation {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    _runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    Ok(arguments.contains_key(&self.identifier))
  }
}
