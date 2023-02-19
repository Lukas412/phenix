use derive_more::{Display, From};
use duplicate::duplicate_item;

use crate::{AnyValue, ComplexCreationArguments, Evaluate, EvaluateError, ExtractTypeFromAnyError, Runtime, ToType};
use crate::evaluate::EvaluateResult;
use crate::operations::GetArgumentOperation;
use crate::value::expression::Expression;

pub type StringExpression = Expression<StringValue, StringOperation>;

#[duplicate_item(FromType; [& str]; [String];)]
impl From<FromType> for StringExpression {
  fn from(value: FromType) -> Self {
    Self::Value(value.into())
  }
}

#[duplicate_item(FromType; [GetArgumentOperation < StringValue >];)]
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

impl TryFrom<AnyValue> for StringValue {
  type Error = EvaluateError;

  fn try_from(value: AnyValue) -> Result<Self, Self::Error> {
    match value {
      AnyValue::String(value) => Ok(value),
      any => Err(ExtractTypeFromAnyError::new(any, ToType::Action).into()),
    }
  }
}

#[derive(Clone, Debug, From)]
pub enum StringOperation {
  #[from(forward)]
  GetArgument(GetArgumentOperation<StringValue>),
}

impl Evaluate for StringOperation {
  type Result = StringValue;

  fn evaluate(
    &self,
    runtime: &Runtime,
    arguments: ComplexCreationArguments,
  ) -> EvaluateResult<Self::Result> {
    match self {
      Self::GetArgument(operation) => operation.evaluate(runtime, arguments),
    }
  }
}
