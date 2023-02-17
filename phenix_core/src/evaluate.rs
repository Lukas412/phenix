use crate::{AnyValue, ComplexCreationArguments, EvaluateErr, Runtime};

pub trait Evaluate<V> {
  fn evaluate(&self, runtime: &Runtime, arguments: ComplexCreationArguments) -> EvaluateResult<V>;
}

pub type EvaluateResult<V> = Result<V, EvaluateErr>;
