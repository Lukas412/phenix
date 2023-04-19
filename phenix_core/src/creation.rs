use derive_more::From;

use crate::AnyExpression;

pub use self::complex::{ComplexCreation, ComplexCreationBuilder, EvaluateArguments};

mod complex;

#[derive(Clone, Debug, From)]
pub enum Creation {
  #[from(forward)]
  Expression(AnyExpression),
  #[from(types(ComplexCreationBuilder))]
  Complex(ComplexCreation),
}

impl Creation {
  pub fn new<IntoAnyExpression>(any_expression: IntoAnyExpression) -> Self
  where
    IntoAnyExpression: Into<AnyExpression>,
  {
    Self::Expression(any_expression.into())
  }
}
