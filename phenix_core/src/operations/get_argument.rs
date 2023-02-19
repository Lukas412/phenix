use crate::{evaluate::EvaluateResult, ArgumentNotFoundError, ComplexCreationArguments, Evaluate, Identifier, Runtime, EvaluateError, Creation, AnyValue};
use std::{fmt::Debug};


#[derive(Clone, Debug)]
pub struct GetArgumentOperation {
  identifier: Identifier,
  default: Option<Creation>,
}

impl GetArgumentOperation {
  pub fn new<I>(identifier: I) -> Self
  where
    I: Into<Identifier>,
  {
    Self {
      identifier: identifier.into(),
      default: None,
    }
  }

  pub fn with_default<I, D>(identifier: I, default: D) -> Self
  where
    I: Into<Identifier>,
    D: Into<Creation>,
  {
    Self {
      identifier: identifier.into(),
      default: Some(default.into()),
    }
  }
}

impl<V> Evaluate<V> for GetArgumentOperation
where V: TryFrom<AnyValue, Error = EvaluateError>
{
  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<V> {
    let creation = arguments.get(&self.identifier)
      .or_else(|| self.default.as_ref())
      .ok_or_else(|| ArgumentNotFoundError::new(self.identifier.clone()))?;

    runtime.evaluate(creation)?.try_into()
  }
}
