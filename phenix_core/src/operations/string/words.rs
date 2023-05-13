use crate::evaluate::EvaluateResult;
use crate::{ContextExt, Evaluate, Runtime, TextExpression, TextJoinOperation, TextValue};

#[derive(Clone, Debug)]
pub struct TextWordsOperation<Context> {
  operation: TextJoinOperation<TextValue, TextExpression<Context>>,
}

impl<Context> TextWordsOperation<Context> {
  pub fn new(expressions: Vec<TextExpression<Context>>) -> Self {
    Self {
      operation: TextJoinOperation::new(" ", expressions),
    }
  }
}

impl<Into1, Into2, Context> From<(Into1, Into2)> for TextWordsOperation<Context>
where
  Into1: Into<TextExpression<Context>>,
  Into2: Into<TextExpression<Context>>,
{
  fn from(values: (Into1, Into2)) -> Self {
    Self::new(vec![values.0.into(), values.1.into()])
  }
}

impl<Into1, Into2, Into3, Context> From<(Into1, Into2, Into3)> for TextWordsOperation<Context>
where
  Into1: Into<TextExpression<Context>>,
  Into2: Into<TextExpression<Context>>,
  Into3: Into<TextExpression<Context>>,
{
  fn from(values: (Into1, Into2, Into3)) -> Self {
    Self::new(vec![values.0.into(), values.1.into(), values.2.into()])
  }
}

impl<Into1, Into2, Into3, Into4, Context> From<(Into1, Into2, Into3, Into4)>
  for TextWordsOperation<Context>
where
  Into1: Into<TextExpression<Context>>,
  Into2: Into<TextExpression<Context>>,
  Into3: Into<TextExpression<Context>>,
  Into4: Into<TextExpression<Context>>,
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

impl<IntoExpression, Context> From<Vec<IntoExpression>> for TextWordsOperation<Context>
where
  IntoExpression: Into<TextExpression<Context>>,
{
  fn from(expressions: Vec<IntoExpression>) -> Self {
    let expressions = expressions.into_iter().map(Into::into).collect();
    Self::new(expressions)
  }
}

impl<Context> Evaluate<Context> for TextWordsOperation<Context>
where
  Context: ContextExt,
{
  type Result = TextValue;

  fn evaluate(&self, runtime: &Runtime, context: &Context) -> EvaluateResult<Self::Result> {
    self.operation.evaluate(runtime, context)
  }
}
