use crate::evaluate::EvaluateResult;
use crate::{
  ActionExpression, ActionValue, AsBash, ContextExt, Evaluate, PathExpression, PathValue, Runtime,
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
pub struct LocationOperation<Context> {
  location: PathExpression<Context>,
  action: Box<ActionExpression<Context>>,
}

impl<Context> LocationOperation<Context> {
  pub fn new<IntoPathExpression, IntoActionExpression>(
    location: IntoPathExpression,
    action: IntoActionExpression,
  ) -> Self
  where
    IntoPathExpression: Into<PathExpression<Context>>,
    IntoActionExpression: Into<ActionExpression<Context>>,
  {
    Self {
      location: location.into(),
      action: Box::new(action.into()),
    }
  }
}

impl<Context> Evaluate<Context> for LocationOperation<Context>
where
  Context: ContextExt,
{
  type Result = LocationValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    Ok(LocationValue::new(
      self.location.evaluate(runtime, context)?,
      self.action.evaluate(runtime, context)?,
    ))
  }
}
