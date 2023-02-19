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

#[duplicate_item(FromType; [GetArgumentOperation];)]
impl From<FromType> for StringExpression {
  fn from(operation: FromType) -> Self {
    Self::Operation(Box::new(operation.into()))
  }
}

#[derive(Clone, Debug, Default, Display, PartialEq, Eq, From)]
#[display(fmt = "\"{value}\"")]
#[from(forward)]
pub struct StringValue {
  value: String,
}

#[derive(Clone, Debug, From)]
pub enum StringOperation {
  GetArgument(GetArgumentOperation),
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
