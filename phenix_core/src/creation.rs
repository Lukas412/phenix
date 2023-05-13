use derive_more::From;

use crate::AnyExpression;

pub use self::complex::{ComplexCreation, ComplexCreationBuilder};

mod complex;

#[derive(Clone, Debug, From)]
pub enum Creation<Context> {
  #[from(forward)]
  Expression(AnyExpression<Context>),
  #[from(types(ComplexCreationBuilder))]
  Complex(ComplexCreation),
}

impl<Context> Creation<Context> {
  pub fn new<IntoAnyExpression>(any_expression: IntoAnyExpression) -> Self
  where
    IntoAnyExpression: Into<AnyExpression<Context>>,
  {
    Self::Expression(any_expression.into())
  }
}
