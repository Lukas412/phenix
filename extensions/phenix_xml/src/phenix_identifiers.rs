use const_str::concat;

pub const XML_ATTRIBUTE: &str = "xml:attribute";
pub const XML_ATTRIBUTE__NAME: &str = concat!(XML_ATTRIBUTE, "$name");
pub const XML_ATTRIBUTE__VALUE: &str = concat!(XML_ATTRIBUTE, "$value");
