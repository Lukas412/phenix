use phenix_core::{ContextExt, Creation, DynamicContext, Identifier, TextExpression};

#[derive(Clone, Debug)]
pub struct NpmRunContext<Context> {
  name: TextExpression<Context>,
  arguments: Option<TextExpression<Context>>,
}

impl<Context> ContextExt for NpmRunContext<Context> {
  fn has(&self, identifier: &Identifier) -> bool {
    if identifier
  }

  fn get(&self, identifier: &Identifier) -> Option<&Creation<Self>> {
    todo!()
  }

  fn len(&self) -> usize {
    todo!()
  }
}
