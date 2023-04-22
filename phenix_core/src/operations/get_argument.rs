use crate::{
  evaluate::EvaluateResult, AnyValue, ArgumentNotFoundError, Creation, Evaluate, EvaluateArguments,
  EvaluateError, Identifier, Runtime,
};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct GetArgumentOperation<T> {
  identifier: Identifier,
  default: Option<Box<Creation>>,
  phantom: PhantomData<T>,
}

impl<T> GetArgumentOperation<T> {
  pub fn new(identifier: impl Into<Identifier>) -> Self {
    Self {
      identifier: identifier.into(),
      default: None,
      phantom: PhantomData,
    }
  }

  pub fn with_default<I, D>(
    identifier: impl Into<Identifier>,
    default: impl Into<Creation>,
  ) -> Self {
    Self {
      identifier: identifier.into(),
      default: Some(Box::new(default.into())),
      phantom: PhantomData,
    }
  }
}

impl<V> Evaluate for GetArgumentOperation<V>
where
  V: TryFrom<AnyValue, Error = EvaluateError>,
{
  type Result = V;

  fn evaluate(&self, runtime: &Runtime, arguments: &EvaluateArguments) -> EvaluateResult<V> {
    arguments
      .get(&self.identifier)
      .or(self.default.as_deref())
      .ok_or_else(|| ArgumentNotFoundError::new(self.identifier.clone()).into())
      .and_then(|creation| runtime.evaluate(creation))?
      .try_into()
  }
}
