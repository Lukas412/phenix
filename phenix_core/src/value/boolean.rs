use derive_more::{Display, From};

use crate::{
  evaluate::EvaluateResult,
  operations::{AndOperation, GetArgumentOperation, OrOperation},
  ComplexCreationArguments, Evaluate, Runtime,
};

#[derive(Clone, Debug, From)]
pub enum BooleanExpression {
  Value(BooleanValue),
  Operation(BooleanOperation),
}

impl Evaluate for BooleanExpression {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::Value(value) => Ok(value.clone()),
      Self::Operation(operation) => operation.evaluate(runtime, arguments),
    }
  }
}

#[derive(Clone, Debug, Display, From)]
pub struct BooleanValue(bool);

#[derive(Clone, Debug)]
pub enum BooleanOperation {
  And(AndOperation<BooleanExpression>),
  Or(OrOperation<BooleanExpression>),
  GetArgument(GetArgumentOperation<BooleanValue>),
}

impl Evaluate for BooleanOperation {
  type Result = BooleanValue;

  fn evaluate(
    &self,
    _runtime: &Runtime,
    _arguments: ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    todo!()
  }
}
