use const_str::concat;

pub const SVELTE: &str = "svelte";

pub const SVELTE_PROJECT: &str = concat!(SVELTE, ":project");
pub const SVELTE_PROJECT__NAME: &str = concat!(SVELTE_PROJECT, "$name");

pub const SVELTE_PROJECT_INIT: &str = concat!(SVELTE_PROJECT, ":init");

pub const SVELTE_PAGE: &str = concat!(SVELTE, ":page");
pub const SVELTE_PAGE__SCRIPT: &str = concat!(SVELTE_PAGE, "$script");

pub const SVELTE_PAGE_CONTENT: &str = concat!(SVELTE_PAGE, ":content");
