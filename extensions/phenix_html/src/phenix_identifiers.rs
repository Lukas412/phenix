use const_str::concat;

pub const HTML: &str = "html";

pub const HTML_ELEMENT: &str = concat!(HTML, ":element");

pub const HTML_ELEMENT_SCRIPT: &str = concat!(HTML_ELEMENT, ":script");
pub const HTML_ELEMENT_SCRIPT__ARGUMENTS: &str = concat!(HTML_ELEMENT_SCRIPT, "$arguments");
pub const HTML_ELEMENT_SCRIPT__CONTENT: &str = concat!(HTML_ELEMENT_SCRIPT, "$content");

pub const HTML_ELEMENT_STYLE: &str = concat!(HTML_ELEMENT, ":style");
pub const HTML_ELEMENT_STYLE__ARGUMENTS: &str = concat!(HTML_ELEMENT_STYLE, "$arguments");
pub const HTML_ELEMENT_STYLE__CONTENT: &str = concat!(HTML_ELEMENT_STYLE, "$content");
