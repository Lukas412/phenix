use itertools::Itertools;

use crate::evaluate::EvaluateResult;
use crate::{ComplexCreationArguments, Evaluate, Runtime, TextValue};

#[derive(Clone, Debug)]
pub struct JoinOperation<Separator, Expression> {
  separator: Box<Separator>,
  expressions: Vec<Expression>,
}

impl<Separator, Expression> JoinOperation<Separator, Expression> {
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

impl<Separator, Expression> Evaluate for JoinOperation<Separator, Expression>
where
  Separator: Evaluate<Result = TextValue>,
  Expression: Evaluate<Result = TextValue>,
{
  type Result = TextValue;

  #[allow(unstable_name_collisions)]
  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: &ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    let separator = self.separator.evaluate(runtime, arguments)?;
    self
      .expressions
      .iter()
      .map(|expression| expression.evaluate(runtime, arguments))
      .intersperse(Ok(separator))
      .collect()
  }
}
