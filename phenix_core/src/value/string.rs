use derive_more::{Display, From};
use duplicate::duplicate_item;

use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::expression::Expression;
use crate::{ComplexCreationArguments, Evaluate, Runtime};
use std::borrow::Cow;

pub type StringExpression = Expression<StringValue, StringOperation>;

#[duplicate_item(FromType; [&str]; [String];)]
impl From<FromType> for StringExpression {
  fn from(value: FromType) -> Self {
    Self::Value(value.into())
  }
}

#[derive(Clone, Debug, Default, Display, PartialEq, Eq)]
#[display(fmt = "\"{value}\"")]
pub struct StringValue {
  value: String,
}

#[duplicate_item(FromType; [&str]; [String];)]
impl From<FromType> for StringValue {
  fn from(value: FromType) -> Self {
    StringValue {
      value: value.into(),
    }
  }
}

#[derive(Clone, Debug)]
pub enum StringOperation {
  GetArgument(GetArgumentOperation<StringExpression>),
}

impl Evaluate<StringValue> for StringOperation {
  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<StringValue> {
    todo!()
  }
}
