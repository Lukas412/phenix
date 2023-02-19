use derive_more::{From};

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::expression::Expression;
use crate::{ComplexCreationArguments, Evaluate, Runtime};

use std::fmt::Display;
use std::path::{PathBuf};

pub type PathExpression = Expression<PathValue, PathOperation>;

#[derive(Clone, Debug, PartialEq, Eq, From)]
#[from(forward)]
pub struct PathValue(PathBuf);

impl Display for PathValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.to_string_lossy())
  }
}

#[derive(Clone, Debug)]
pub enum PathOperation {
  GetArgument(GetArgumentOperation),
}

impl Evaluate<PathValue> for PathOperation {
  fn evaluate(
    &self,
    _runtime: &Runtime,
    _arguments: ComplexCreationArguments,
  ) -> EvaluateResult<PathValue> {
    todo!()
  }
}
