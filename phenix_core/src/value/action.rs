use derive_more::{Display, From};

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::array::ArrayValue;
use crate::value::expression::Expression;
use crate::value::path::PathValue;
use crate::{
  AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, PathExpression, Runtime,
  StringExpression, StringValue,
};

pub type ActionExpression = Expression<ActionValue, ActionOperation>;

#[derive(Clone, Debug, From)]
pub enum ActionValue {
  ChangeLocation {
    location: PathExpression,
    actions: ArrayValue<ActionExpression>,
  },
  ExecuteCommand {
    name: StringExpression,
    arguments: ArrayValue<StringExpression>,
  },
  WriteContent {
    file: PathExpression,
    content: StringExpression,
  },
  EnsureDirectory {
    file: PathExpression,
  },
}

#[derive(Clone, Debug)]
pub enum ActionOperation {
  GetArgument(GetArgumentOperation),
}

impl Evaluate<ActionValue> for ActionOperation {
  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<ActionValue> {
    todo!()
  }
}
