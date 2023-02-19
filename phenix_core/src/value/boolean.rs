use derive_more::{Display, From};

use super::Expression;
use crate::{
  evaluate::EvaluateResult,
  operations::{
    AndOperation, EvaluateAdd, EvaluateAnd, EvaluateOr, GetArgumentOperation, OrOperation,
  },
  ComplexCreationArguments, Evaluate, Runtime,
};

pub type BooleanExpression = Expression<BooleanValue, BooleanOperation>;

impl EvaluateAnd for BooleanExpression {
  type Output = BooleanValue;

  fn evaluate_and(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

impl EvaluateOr for BooleanExpression {
  type Output = BooleanValue;

  fn evaluate_or(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

#[derive(Clone, Debug, Display, From)]
pub struct BooleanValue(bool);

#[derive(Clone, Debug)]
pub enum BooleanOperation {
  And(AndOperation<BooleanExpression>),
  Or(OrOperation<BooleanExpression>),
  GetArgument(GetArgumentOperation),
}

impl Evaluate<BooleanValue> for BooleanOperation {
  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<BooleanValue> {
    todo!()
  }
}
