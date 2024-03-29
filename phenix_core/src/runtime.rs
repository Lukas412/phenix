use std::collections::HashMap;

use crate::evaluate::EvaluateResult;
use crate::{
  ActionExpression, AnyExpression, AnyValue, BooleanExpression, ContextExt, Creation,
  DynamicContext, Evaluate, ExpressionNotFoundError, Namespace, NumberExpression, PathExpression,
  TextExpression,
};

#[derive(Debug, Default)]
pub struct Runtime {
  values: HashMap<Namespace, AnyExpression<DynamicContext>>,
}

impl Runtime {
  pub fn evaluate<'b, Context>(
    &'b self,
    creation: &'b Creation<Context>,
  ) -> EvaluateResult<AnyValue>
  where
    Context: Default,
    Context: ContextExt,
  {
    match creation {
      Creation::Expression(expression) => {
        let arguments = Context::default();
        expression.evaluate(self, &arguments)
      }
      Creation::Complex(complex) => self
        .get_expression(complex.namespace())?
        .evaluate(self, complex.values()),
    }
  }

  fn get_expression(
    &self,
    namespace: &Namespace,
  ) -> Result<&AnyExpression<DynamicContext>, ExpressionNotFoundError> {
    self
      .values
      .get(namespace)
      .ok_or_else(|| ExpressionNotFoundError::new(namespace.to_owned()))
  }

  fn new(values: HashMap<Namespace, AnyExpression<DynamicContext>>) -> Self {
    Self { values }
  }
}

#[derive(Default)]
pub struct RuntimeBuilder {
  values: HashMap<Namespace, AnyExpression<DynamicContext>>,
}

impl RuntimeBuilder {
  pub fn build(self) -> Runtime {
    Runtime::new(self.values)
  }

  pub fn with_action<N, E>(self, namespace: N, action: E) -> Self
  where
    N: Into<Namespace>,
    E: Into<ActionExpression<DynamicContext>>,
  {
    self.with_expression(namespace, action.into())
  }

  pub fn with_boolean<N, E>(self, namespace: N, boolean: E) -> Self
  where
    N: Into<Namespace>,
    E: Into<BooleanExpression>,
  {
    self.with_expression(namespace, boolean.into())
  }

  pub fn with_number<N, E>(self, namespace: N, number: E) -> Self
  where
    N: Into<Namespace>,
    E: Into<NumberExpression>,
  {
    self.with_expression(namespace, number.into())
  }

  pub fn with_path<N, E>(self, namespace: N, path: E) -> Self
  where
    N: Into<Namespace>,
    E: Into<PathExpression<DynamicContext>>,
  {
    self.with_expression(namespace, path.into())
  }

  pub fn with_text<N, E>(self, namespace: N, string: E) -> Self
  where
    N: Into<Namespace>,
    E: Into<TextExpression<DynamicContext>>,
  {
    self.with_expression(namespace, string.into())
  }

  pub fn with_expression<N, E>(mut self, namespace: N, value: E) -> Self
  where
    N: Into<Namespace>,
    E: Into<AnyExpression<DynamicContext>>,
  {
    self.values.insert(namespace.into(), value.into());
    self
  }
}
