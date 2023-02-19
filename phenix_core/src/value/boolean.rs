use derive_more::{Display, From};

use super::Expression;
use crate::{
  evaluate::EvaluateResult,
  operations::{
    AndOperation, EvaluateAnd, EvaluateOr, GetArgumentOperation, OrOperation,
  },
  ComplexCreationArguments, Evaluate, Runtime,
};

pub type BooleanExpression = Expression<BooleanValue, BooleanOperation>;

impl EvaluateAnd for BooleanExpression {
  type Output = BooleanValue;

  fn evaluate_and(self, _rhs: Self) -> Self::Output {
    todo!()
  }
}

impl EvaluateOr for BooleanExpression {
  type Output = BooleanValue;

  fn evaluate_or(self, _rhs: Self) -> Self::Output {
    todo!()
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
