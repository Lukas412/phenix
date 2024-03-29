use crate::{GIT_INIT__DIRECTORY, GIT_INIT__QUIET};
use phenix_core::{
  ActionOperation, AndOperation, BooleanExpression, CommandOperation, ConditionOperation,
  ContextSwitchOperation, Creation, DynamicContext, GetArgumentOperation, HasArgumentOperation,
  TextExpression,
};

pub fn new_git_init_with<Context>(
  directory: Option<impl Into<TextExpression<Context>>>,
  quiet: Option<impl Into<BooleanExpression>>,
) -> ActionOperation<Context> {
  ContextSwitchOperation::new(
    new_git_init_context(directory.map(Into::into), quiet.map(Into::into)),
    new_git_init(),
  )
  .into()
}

pub fn new_git_init<Context>() -> CommandOperation<Context> {
  (
    "git init",
    ConditionOperation::new(
      AndOperation::new(
        HasArgumentOperation::new(GIT_INIT__QUIET),
        GetArgumentOperation::new(GIT_INIT__QUIET),
      ),
      "--quiet",
      "",
    ),
    ConditionOperation::new(
      HasArgumentOperation::new(GIT_INIT__DIRECTORY),
      GetArgumentOperation::new(GIT_INIT__DIRECTORY),
      "",
    ),
  )
    .into()
}

fn new_git_init_context<Context>(
  directory: Option<impl Into<Creation<Context>>>,
  quiet: Option<impl Into<Creation<Context>>>,
) -> DynamicContext {
  let mut context = DynamicContext::with_capacity(2);
  if let Some(directory) = directory {
    context.insert(GIT_INIT__DIRECTORY.into(), directory.into());
  }
  if let Some(quiet) = quiet {
    context.insert(GIT_INIT__QUIET.into(), quiet.into());
  }
  context
}
