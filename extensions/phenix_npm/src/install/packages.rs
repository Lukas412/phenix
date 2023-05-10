use crate::NPM_INSTALL__PACKAGES;
use phenix_core::{
  ActionOperation, CommandOperation, ContextSwitchOperation, Creation, DynamicContext,
  GetArgumentOperation, TextExpression,
};

pub fn new_npm_install_packages_with(packages: impl Into<TextExpression>) -> ActionOperation {
  ContextSwitchOperation::new(
    new_npm_install_packages_context(packages.into()),
    new_npm_install_packages(),
  )
  .into()
}

pub fn new_npm_install_packages() -> CommandOperation {
  CommandOperation::from((
    "npm install",
    GetArgumentOperation::new(NPM_INSTALL__PACKAGES),
  ))
}

fn new_npm_install_packages_context(packages: impl Into<Creation>) -> DynamicContext {
  [(NPM_INSTALL__PACKAGES.into(), packages.into())].into()
}
