use crate::{ActionValue, PathValue};

pub struct ChangeLocationValue {
  location: PathValue,
  actions: Box<ActionValue>,
}
