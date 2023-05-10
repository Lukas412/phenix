use crate::evaluate::EvaluateResult;
use crate::{BooleanValue, Evaluate, Identifier, Runtime};

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

impl<Context> Evaluate<Context> for HasArgumentOperation {
  type Result = BooleanValue;

  fn evaluate(&self, _runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    Ok(context.contains_key(&self.identifier))
  }
}
