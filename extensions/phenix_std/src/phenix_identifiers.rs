use const_str::concat;

pub const STD: &str = "std";

pub const STD_TEXT: &str = concat!(STD, ":text");
pub const STD_TEXT__VALUE: &str = concat!(STD_TEXT, "$value");

pub const STD_TEXT_EMPTY: &str = concat!(STD_TEXT, ":empty");

pub const STD_TEXT_QUOTED: &str = concat!(STD_TEXT, ":quoted");
pub const STD_TEXT_QUOTED_SINGLE: &str = concat!(STD_TEXT_QUOTED, ":single");
pub const STD_TEXT_QUOTED_DOUBLE: &str = concat!(STD_TEXT_QUOTED, ":double");
