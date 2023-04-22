use crate::{XML_ATTRIBUTE__NAME, XML_ATTRIBUTE__VALUE};
use phenix_core::{
  ContextSwitchOperation, Creation, EvaluateArguments, GetArgumentOperation, TextBlockOperation,
  TextExpression, TextOperation,
};
use phenix_std::{new_std_text_quoted_double_operation, new_std_text_with};

pub fn new_xml_attribute_with(
  name: impl Into<TextExpression>,
  value: impl Into<TextExpression>,
) -> TextOperation {
  ContextSwitchOperation::new(
    new_xml_attribute_context(name.into(), value.into()),
    new_xml_attribute_operation(),
  )
  .into()
}

pub fn new_xml_attribute_operation() -> TextOperation {
  TextBlockOperation::from((
    GetArgumentOperation::new(XML_ATTRIBUTE__NAME),
    "=",
    new_std_text_with(
      GetArgumentOperation::new(XML_ATTRIBUTE__VALUE),
      new_std_text_quoted_double_operation(),
    ),
  ))
  .into()
}

fn new_xml_attribute_context(
  name: impl Into<Creation>,
  value: impl Into<Creation>,
) -> EvaluateArguments {
  [
    (XML_ATTRIBUTE__NAME.into(), name.into()),
    (XML_ATTRIBUTE__VALUE.into(), value.into()),
  ]
  .into()
}
