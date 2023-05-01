use crate::STD_TEXT__VALUE;
use phenix_core::{
  ContextSwitchOperation, Creation, EvaluateArguments, TextExpression, TextOperation,
};
pub use {empty::new_std_text_empty, quoted::new_std_text_quoted_double_operation};

mod empty;
mod quoted;

pub fn new_std_text_with(
  value: impl Into<TextExpression>,
  expression: impl Into<TextExpression>,
) -> TextOperation {
  ContextSwitchOperation::new(new_std_text_context(value.into()), expression).into()
}

fn new_std_text_context(value: impl Into<Creation>) -> EvaluateArguments {
  [(STD_TEXT__VALUE.into(), value.into())].into()
}
