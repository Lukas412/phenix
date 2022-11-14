use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanValue {
  True,
  False,
}

impl ValueExt for BooleanValue {
  fn is_concrete(&self) -> bool {
    matches!(self, Self::True | Self::False)
  }
}

impl From<bool> for BooleanValue {
  fn from(boolean: bool) -> Self {
    match boolean {
      true => Self::True,
      false => Self::False,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from {
    use super::*;

    #[test]
    fn from_true() {
      let expected = BooleanValue::True;
      let actual = true.into();
      assert_eq!(expected, actual);
    }

    #[test]
    fn from_false() {
      let expected = BooleanValue::False;
      let actual = false.into();
      assert_eq!(expected, actual);
    }
  }
}
