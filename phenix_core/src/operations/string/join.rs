use itertools::Itertools;

use crate::evaluate::EvaluateResult;
use crate::{Evaluate, EvaluateArguments, Runtime, TextValue};

#[derive(Clone, Debug)]
pub struct TextJoinOperation<Separator, Expression> {
  separator: Box<Separator>,
  expressions: Vec<Expression>,
}

impl<Separator, Expression> TextJoinOperation<Separator, Expression> {
  pub fn new<IntoSeparator>(separator: IntoSeparator, expressions: Vec<Expression>) -> Self
  where
    IntoSeparator: Into<Separator>,
  {
    Self {
      separator: Box::new(separator.into()),
      expressions,
    }
  }
}

impl<Separator, Expression> Evaluate for TextJoinOperation<Separator, Expression>
where
  Separator: Evaluate<Result = TextValue>,
  Expression: Evaluate<Result = TextValue>,
{
  type Result = TextValue;

  #[allow(unstable_name_collisions)]
  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &EvaluateArguments,
  ) -> EvaluateResult<Self::Result> {
    let separator = self.separator.evaluate(runtime, arguments)?;
    self
      .expressions
      .iter()
      .map(|expression| expression.evaluate(runtime, arguments))
      .filter_ok(|value| !value.is_empty())
      .intersperse(Ok(separator))
      .collect()
  }
}
