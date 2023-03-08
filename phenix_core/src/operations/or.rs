use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct OrOperation<Expression, OtherExpression = Expression> {
  expressions: Box<(Expression, OtherExpression)>,
}

impl<Expression, OtherExpression> OrOperation<Expression, OtherExpression> {
  pub fn new<IntoExpression, IntoOtherExpression>(expression: IntoExpression, other_expression: IntoOtherExpression) -> Self
    where
      IntoExpression: Into<Expression>,
      IntoOtherExpression: Into<OtherExpression>,
  {
    Self {
      expressions: (expression.into(), other_expression.into()).into()
    }
  }
}
