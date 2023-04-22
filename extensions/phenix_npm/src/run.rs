use phenix_core::{
  ActionOperation, CommandOperation, ConditionOperation, ContextSwitchOperation, Creation,
  EvaluateArguments, GetArgumentOperation, HasArgumentOperation, TextWordsOperation,
};

pub const NPM_RUN: &str = "npm:run";
pub const NPM_RUN__NAME: &str = "npm:run$name";
pub const NPM_RUN__ARGUMENTS: &str = "npm:run$arguments";

pub fn new_npm_run_command_operation_with_context_switch(
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
