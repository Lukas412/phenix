pub use crate::{
  bash::AsBash,
  creation::{ComplexCreationBuilder, Creation},
  error::{
    ArgumentNotFoundError, EvaluateError, ExpressionNotFoundError, ExtractTypeFromAnyError,
    GetArgumentOperationError, ToType,
  },
  evaluate::{Evaluate, EvaluateArguments},
  names::{Identifier, Name, Namespace},
  operations::{
    AddOperation, AndOperation, ConditionOperation, ContextExtendOperation, ContextSwitchOperation,
    EqualsOperation, GetArgumentOperation, HasArgumentOperation, OrOperation, PathJoinOperation,
    SubOperation, TextBlockOperation, TextJoinOperation, TextLinesOperation, TextWordsOperation,
    ToBooleanOperation, ToPathOperation,
  },
  runtime::{Runtime, RuntimeBuilder},
  value::{
    ActionExpression, ActionOperation, ActionValue, AnyExpression, AnyValue, BooleanExpression,
    BooleanOperation, BooleanValue, CommandOperation, CommandValue, LocationOperation,
    LocationValue, NumberExpression, NumberOperation, NumberValue, PathExpression, PathOperation,
    PathValue, TextExpression, TextOperation, TextValue,
  },
};

mod bash;
mod creation;
mod error;
mod evaluate;
mod names;
mod operations;
mod runtime;
mod value;
