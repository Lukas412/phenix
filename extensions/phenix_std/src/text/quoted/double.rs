use phenix_core::{GetArgumentOperation, TextBlockOperation, TextOperation};

use crate::STD_TEXT__VALUE;

pub fn new_std_text_quoted_double_operation() -> TextOperation {
  TextBlockOperation::from(("\"", GetArgumentOperation::new(STD_TEXT__VALUE), "\"")).into()
}
