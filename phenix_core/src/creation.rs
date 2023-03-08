use derive_more::From;

use crate::AnyExpression;

pub use self::complex::{ComplexCreation, ComplexCreationArguments, ComplexCreationBuilder};

mod complex;

#[derive(Clone, Debug, From)]
pub enum Creation {
  #[from(forward)]
  Expression(AnyExpression),
  #[from(types(ComplexCreationBuilder))]
  Complex(ComplexCreation),
}
