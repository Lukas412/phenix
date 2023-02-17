pub use self::{
  evaluate::EvaluateErr,
  expression::ExpressionNotFoundError,
  operation::{ArgumentNotFoundError, GetArgumentOperationError},
};

mod evaluate;
mod expression;
mod operation;
