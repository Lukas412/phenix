use crate::NPM_INSTALL__PACKAGES;
use phenix_core::{
  ActionOperation, CommandOperation, ContextSwitchOperation, Creation, DynamicContext,
  GetArgumentOperation, TextExpression,
};

pub fn new_npm_install_packages_with<Context>(
  packages: impl Into<TextExpression<Context>>,
) -> ActionOperation<Context> {
  ContextSwitchOperation::new(
    new_npm_install_packages_context(packages.into()),
    new_npm_install_packages(),
  )
  .into()
}

pub fn new_npm_install_packages<Context>() -> CommandOperation<Context> {
  CommandOperation::from((
    "npm install",
    GetArgumentOperation::new(NPM_INSTALL__PACKAGES),
  ))
}

fn new_npm_install_packages_context<Context>(
  packages: impl Into<Creation<Context>>,
) -> DynamicContext {
  [(NPM_INSTALL__PACKAGES.into(), packages.into())].into()
}
