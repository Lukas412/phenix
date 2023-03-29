use crate::BooleanExpression;

pub struct ConditionOperation<Expression> {
  condition: BooleanExpression,
  then: Expression,
  other: Expression,
}
