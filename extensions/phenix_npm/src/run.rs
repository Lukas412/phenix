use crate::{NPM_RUN__ARGUMENTS, NPM_RUN__NAME};
use phenix_core::{
  ActionOperation, CommandOperation, ConditionOperation, ContextSwitchOperation, Creation,
  EvaluateArguments, GetArgumentOperation, HasArgumentOperation, TextWordsOperation,
};

pub fn new_npm_run_command_with(
  name: impl Into<Creation>,
  arguments: Option<impl Into<Creation>>,
) -> ActionOperation {
  ContextSwitchOperation::new(
    new_npm_run_context(name, arguments),
    new_npm_run_command_operation(),
  )
  .into()
}

pub fn new_npm_run_command_operation() -> CommandOperation {
  (
    "npm run",
    GetArgumentOperation::new(NPM_RUN__NAME),
    ConditionOperation::new(
      HasArgumentOperation::new(NPM_RUN__ARGUMENTS),
      TextWordsOperation::from(("--", GetArgumentOperation::new(NPM_RUN__ARGUMENTS))),
      "",
    ),
  )
    .into()
}

fn new_npm_run_context(
  name: impl Into<Creation>,
  arguments: Option<impl Into<Creation>>,
) -> EvaluateArguments {
  match arguments {
    Some(arguments) => [
      (NPM_RUN__NAME.into(), name.into()),
      (NPM_RUN__ARGUMENTS.into(), arguments.into()),
    ]
    .into(),
    None => [(NPM_RUN__NAME.into(), name.into())].into(),
  }
}
