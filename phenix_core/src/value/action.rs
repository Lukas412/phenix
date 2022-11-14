use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionValue {}

impl ValueExt for ActionValue {
  fn is_concrete(&self) -> bool {
    matches!(self, _)
  }
}

#[cfg(test)]
mod tests {}
