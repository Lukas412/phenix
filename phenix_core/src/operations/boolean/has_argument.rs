use crate::evaluate::EvaluateResult;
use crate::{BooleanValue, ContextExt, Evaluate, Identifier, Runtime};

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

impl<Context> Evaluate<Context> for HasArgumentOperation
where
  Context: ContextExt,
{
  type Result = BooleanValue;

  fn evaluate(&self, _runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    Ok(context.has(&self.identifier))
  }
}
