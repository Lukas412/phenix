use crate::{
  evaluate::EvaluateResult, AnyValue, ArgumentNotFoundError, ContextExt, Evaluate, EvaluateError,
  Identifier, Runtime,
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

impl<Value, Context> Evaluate<Context> for GetArgumentOperation<Value>
where
  Value: TryFrom<AnyValue, Error = EvaluateError>,
  Context: ContextExt,
{
  type Result = Value;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Value> {
    context
      .get(&self.identifier)
      .ok_or_else(|| ArgumentNotFoundError::new(self.identifier.clone()).into())
      .and_then(|creation| runtime.evaluate(creation))?
      .try_into()
  }
}
