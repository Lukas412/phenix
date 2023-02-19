use derive_more::{From};

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::array::ArrayValue;
use crate::value::expression::Expression;

use crate::{
  ComplexCreationArguments, Evaluate, PathExpression, Runtime,
  StringExpression,
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
    _runtime: &Runtime,
    _arguments: ComplexCreationArguments,
  ) -> EvaluateResult<ActionValue> {
    todo!()
  }
}
