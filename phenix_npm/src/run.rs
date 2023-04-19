use phenix_core::{
  CommandOperation, ConditionOperation, GetArgumentOperation, HasArgumentOperation,
  TextWordsOperation,
};

pub const NPM_RUN: &str = "npm:run";
pub const NPM_RUN__NAME: &str = "npm:run$name";
pub const NPM_RUN__ARGUMENTS: &str = "npm:run$arguments";

pub fn new_npm_run_command_operation() -> CommandOperation {
  CommandOperation::from((
    "npm",
    GetArgumentOperation::new(NPM_RUN__NAME),
    ConditionOperation::new(
      HasArgumentOperation::new(NPM_RUN__ARGUMENTS),
      TextWordsOperation::from(("--", GetArgumentOperation::new(NPM_RUN__ARGUMENTS))),
      "",
    ),
  ))
}
