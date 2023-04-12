use crate::BooleanExpression;

#[allow(unused)]
pub struct ConditionOperation<Expression> {
  condition: BooleanExpression,
  then: Expression,
  other: Expression,
}
