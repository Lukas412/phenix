use std::{borrow::Cow, collections::HashMap, path::PathBuf, rc::Rc};

use rust_decimal::Decimal;

use crate::{Creation, Value};

pub trait ValueExt<'a>: Into<Value> + Clone {
  fn eval(&self, arguments: Rc<HashMap<Cow<'a, str>, Creation<'a>>>) -> Result<Value, String>;

  fn to_bool(self) -> Option<bool>;
  fn to_int(self) -> Option<i32>;
  fn to_decimal(self) -> Option<Decimal>;
  fn to_path(self) -> Option<PathBuf>;
  fn to_string(self) -> Option<String>;
}
