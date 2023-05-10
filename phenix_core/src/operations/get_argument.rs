use crate::{
  evaluate::EvaluateResult, AnyValue, ArgumentNotFoundError, Creation, Evaluate, EvaluateArguments,
  EvaluateError, Identifier, Runtime,
};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct GetArgumentOperation<T> {
  identifier: Identifier,
  phantom: PhantomData<T>,
}

impl<T> GetArgumentOperation<T> {
  pub fn new(identifier: impl Into<Identifier>) -> Self {
    Self {
      identifier: identifier.into(),
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
      .ok_or_else(|| ArgumentNotFoundError::new(self.identifier.clone()).into())
      .and_then(|creation| runtime.evaluate(creation))?
      .try_into()
  }
}
