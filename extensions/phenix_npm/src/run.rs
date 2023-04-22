use phenix_core::{
  CommandOperation, ConditionOperation, ContextSwitchOperation, Creation, EvaluateArguments,
  GetArgumentOperation, HasArgumentOperation, TextWordsOperation,
};

pub const NPM_RUN: &str = "npm:run";
pub const NPM_RUN__NAME: &str = "npm:run$name";
pub const NPM_RUN__ARGUMENTS: &str = "npm:run$arguments";

pub fn new_npm_run_command_operation(
  name: impl Into<Creation>,
  arguments: Option<impl Into<Creation>>,
) -> impl Into<CommandOperation> {
  ContextSwitchOperation::new(
    new_npm_run_context(name, arguments),
    CommandOperation::from((
      "npm run",
      GetArgumentOperation::new(NPM_RUN__NAME),
      ConditionOperation::new(
        HasArgumentOperation::new(NPM_RUN__ARGUMENTS),
        TextWordsOperation::from(("--", GetArgumentOperation::new(NPM_RUN__ARGUMENTS))),
        "",
      ),
    )),
  )
}

fn new_npm_run_context(
  name: impl Into<Creation>,
  arguments: Option<impl Into<Creation>>,
) -> impl Into<EvaluateArguments> {
  match arguments {
    Some(arguments) => [
      (NPM_RUN__NAME.into(), name.into()),
      (NPM_RUN__ARGUMENTS.into(), arguments.into()),
    ],
    None => [(NPM_RUN__NAME.into(), name.into())],
  }
}
