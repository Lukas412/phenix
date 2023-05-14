use crate::{NPM_RUN__ARGUMENTS, NPM_RUN__NAME};
pub use context::NpmRunContext;
use phenix_core::{
  ActionOperation, CommandOperation, ConditionOperation, ContextSwitchOperation, Creation,
  DynamicContext, GetArgumentOperation, HasArgumentOperation, TextWordsOperation,
};

mod context;

pub fn new_npm_run_with<Context>(
  name: impl Into<Creation<Context>>,
  arguments: Option<impl Into<Creation<Context>>>,
) -> ActionOperation<Context> {
  ContextSwitchOperation::new(
    new_npm_run_context::<Context>(name, arguments),
    new_npm_run_command_operation(),
  )
  .into()
}

pub fn new_npm_run_command_operation<Context>() -> CommandOperation<Context> {
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

fn new_npm_run_context<Context>(
  name: impl Into<Creation<Context>>,
  arguments: Option<impl Into<Creation<Context>>>,
) -> DynamicContext {
  match arguments {
    Some(arguments) => [
      (NPM_RUN__NAME.into(), name.into()),
      (NPM_RUN__ARGUMENTS.into(), arguments.into()),
    ]
    .into(),
    None => [(NPM_RUN__NAME.into(), name.into())].into(),
  }
}
