use const_str::concat;

pub const GIT: &str = "git";

pub const GIT_INIT: &str = concat!(GIT, ":init");
pub const GIT_INIT__QUIET: &str = concat!(GIT_INIT, "$quiet");
pub const GIT_INIT__DIRECTORY: &str = concat!(GIT_INIT, "$directory");
