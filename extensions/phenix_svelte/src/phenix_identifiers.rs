use const_str::concat;

pub const SVELTE: &str = "svelte";

pub const SVELTE_PROJECT: &str = concat!(SVELTE, ":project");
pub const SVELTE_PROJECT__NAME: &str = concat!(SVELTE_PROJECT, "$name");

pub const SVELTE_PROJECT_INIT: &str = concat!(SVELTE_PROJECT, ":init");

pub const SVELTE_COMPONENT: &str = concat!(SVELTE, ":component");
pub const SVELTE_COMPONENT__BODY: &str = concat!(SVELTE_COMPONENT, "$body");

pub const SVELTE_COMPONENT_CONTENT: &str = concat!(SVELTE_COMPONENT, ":content");

pub const SVELTE_COMPONENT_SCRIPT: &str = concat!(SVELTE_COMPONENT, ":script");
pub const SVELTE_COMPONENT_SCRIPT__CONTENT: &str = concat!(SVELTE_COMPONENT_SCRIPT, "$content");

pub const SVELTE_COMPONENT_STYLE: &str = concat!(SVELTE_COMPONENT, ":style");
pub const SVELTE_COMPONENT_STYLE__CONTENT: &str = concat!(SVELTE_COMPONENT_STYLE, "$content");
