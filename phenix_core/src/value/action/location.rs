use crate::evaluate::EvaluateResult;
use crate::{
  ActionExpression, ActionValue, AsBash, Evaluate, EvaluateArguments, PathExpression, PathValue,
  Runtime,
};

#[derive(Clone, Debug)]
pub struct LocationValue {
  location: PathValue,
  action: Box<ActionValue>,
}

impl LocationValue {
  pub fn new<IntoPathValue, IntoActionValue>(
    location: IntoPathValue,
    action: IntoActionValue,
  ) -> Self
  where
    IntoPathValue: Into<PathValue>,
    IntoActionValue: Into<ActionValue>,
  {
    Self {
      location: location.into(),
      action: Box::new(action.into()),
    }
  }
}

impl AsBash for LocationValue {
  fn as_bash(&self) -> String {
    format!(
      "pushd {}\n{}\npopd",
      self.location.display(),
      self.action.as_bash()
    )
  }
}

#[derive(Clone, Debug)]
pub struct LocationOperation {
  location: PathExpression,
  action: Box<ActionExpression>,
}

impl LocationOperation {
  pub fn new<IntoPathExpression, IntoActionExpression>(
    location: IntoPathExpression,
    action: IntoActionExpression,
  ) -> Self
  where
    IntoPathExpression: Into<PathExpression>,
    IntoActionExpression: Into<ActionExpression>,
  {
    Self {
      location: location.into(),
      action: Box::new(action.into()),
    }
  }
}

impl Evaluate for LocationOperation {
  type Result = LocationValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    Ok(LocationValue::new(
      self.location.evaluate(runtime, arguments)?,
      self.action.evaluate(runtime, arguments)?,
    ))
  }
}
