use const_str::concat;

pub const NPM: &str = "npm";

pub const NPM_INSTALL: &str = concat!(NPM, ":install");
pub const NPM_INSTALL__PACKAGES: &str = concat!(NPM_INSTALL, "$packages");

pub const NPM_RUN: &str = concat!(NPM, ":run");
pub const NPM_RUN__NAME: &str = concat!(NPM_RUN, "$name");
pub const NPM_RUN__ARGUMENTS: &str = concat!(NPM_RUN, "$arguments");
