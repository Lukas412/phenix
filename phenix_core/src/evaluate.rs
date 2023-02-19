use crate::{ComplexCreationArguments, EvaluateError, Runtime};

pub trait Evaluate {
  type Result;

  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<Self::Result>;
}

pub type EvaluateResult<V> = Result<V, EvaluateError>;
