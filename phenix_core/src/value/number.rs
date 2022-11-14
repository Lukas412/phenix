use super::ValueExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberValue {
  Int(i32),
  // Float(f32),
}

impl ValueExt for NumberValue {
  fn is_concrete(&self) -> bool {
    matches!(self, Self::Int(_))
  }
}

impl From<i32> for NumberValue {
  fn from(number: i32) -> Self {
    Self::Int(number)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod from {
    use super::*;

    #[test]
    fn from_i32() {
      let expected = NumberValue::Int(1);
      let actual = 1.into();
      assert_eq!(expected, actual);
    }
  }

  mod value_ext {
    use super::*;

    #[test]
    fn is_concrete() {
      let value: NumberValue = 1.into();
      assert!(value.is_concrete());
    }
  }
}
