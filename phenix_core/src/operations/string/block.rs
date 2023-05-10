use crate::evaluate::EvaluateResult;
use crate::{DynamicContext, Evaluate, Runtime, TextExpression, TextValue};

#[derive(Clone, Debug)]
pub struct TextBlockOperation {
  expressions: Vec<TextExpression>,
}

impl TextBlockOperation {
  pub fn new(expressions: Vec<TextExpression>) -> Self {
    Self { expressions }
  }
}

impl<Into1, Into2> From<(Into1, Into2)> for TextBlockOperation
where
  Into1: Into<TextExpression>,
  Into2: Into<TextExpression>,
{
  fn from(values: (Into1, Into2)) -> Self {
    Self::new(vec![values.0.into(), values.1.into()])
  }
}

impl<Into1, Into2, Into3> From<(Into1, Into2, Into3)> for TextBlockOperation
where
  Into1: Into<TextExpression>,
  Into2: Into<TextExpression>,
  Into3: Into<TextExpression>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::new(vec![values.0.into(), values.1.into(), values.2.into()])
  }
}

impl<Into1, Into2, Into3, Into4> From<(Into1, Into2, Into3, Into4)> for TextBlockOperation
where
  Into1: Into<TextExpression>,
  Into2: Into<TextExpression>,
  Into3: Into<TextExpression>,
  Into4: Into<TextExpression>,
{
  fn from(values: (Into1, Into2, Into3, Into4)) -> Self {
    Self::new(vec![
      values.0.into(),
      values.1.into(),
      values.2.into(),
      values.3.into(),
    ])
  }
}

impl<IntoExpression, Context> From<Vec<IntoExpression>> for TextBlockOperation
where
  IntoExpression: Into<TextExpression<Context>>,
{
  fn from(expressions: Vec<IntoExpression>) -> Self {
    let expressions = expressions.into_iter().map(Into::into).collect();
    Self::new(expressions)
  }
}

impl<Context> Evaluate<Context> for TextBlockOperation {
  type Result = TextValue;

  fn evaluate(&self, runtime: &Runtime, arguments: &Context) -> EvaluateResult<Self::Result> {
    self
      .expressions
      .iter()
      .map(|expression| expression.evaluate(runtime, arguments))
      .collect()
  }
}
