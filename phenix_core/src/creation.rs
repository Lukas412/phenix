

use derive_more::From;

use crate::{AnyExpression};

pub use self::complex::{ComplexCreation, ComplexCreationArguments, ComplexCreationBuilder};

mod complex;

#[derive(Clone, Debug, From)]
pub enum Creation {
  #[from(forward)]
  Expression(AnyExpression),
  Complex(ComplexCreation),
}

impl From<ComplexCreation> for Creation {
  fn from(creation: ComplexCreation) -> Self {
    Self::Complex(creation)
  }
}

impl From<ComplexCreationBuilder> for Creation {
  fn from(builder: ComplexCreationBuilder) -> Self {
    builder.build().into()
  }
}
