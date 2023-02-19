use crate::{evaluate::EvaluateResult, ArgumentNotFoundError, ComplexCreationArguments, Evaluate, Identifier, Runtime, EvaluateError, Creation, AnyValue};
use std::{fmt::Debug};
use std::marker::PhantomData;


#[derive(Clone, Debug)]
pub struct GetArgumentOperation<T> {
  identifier: Identifier,
  default: Option<Creation>,
  phantom: PhantomData<T>
}

impl<T> GetArgumentOperation<T> {
  pub fn new<I>(identifier: I) -> Self
  where
    I: Into<Identifier>,
  {
    Self {
      identifier: identifier.into(),
      default: None,
      phantom: PhantomData
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
      phantom: PhantomData
    }
  }
}

impl<T> From<Identifier> for GetArgumentOperation<T> {
  fn from(identifier: Identifier) -> Self {
    Self::new(identifier)
  }
}

impl<V> Evaluate for GetArgumentOperation<V>
where V: TryFrom<AnyValue, Error = EvaluateError>
{
  type Result = V;

  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<V> {
    let creation = arguments.get(&self.identifier)
      .or(self.default.as_ref())
      .ok_or_else(|| ArgumentNotFoundError::new(self.identifier.clone()))?;

    runtime.evaluate(creation)?.try_into()
  }
}
