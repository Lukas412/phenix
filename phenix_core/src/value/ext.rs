use std::{collections::HashMap, path::PathBuf, rc::Rc};

use crate::{Creation, Value};

pub trait ValueExt: Into<Value> + Clone {
  fn eval<'a>(&self, arguments: Rc<HashMap<&'a str, Creation<'a>>>) -> Result<Value, String>;

  fn to_bool(self) -> Option<bool>;
  fn to_int(self) -> Option<i32>;
  fn to_float(self) -> Option<f32>;
  fn to_path(self) -> Option<PathBuf>;
  fn to_string(self) -> Option<String>;
}
