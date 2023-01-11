use std::{borrow::Cow, path::Path};

use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConcreteValue<'a> {
  Boolean(ConcreteBooleanValue),
  Number(ConcreteNumberValue),
  Path(ConcretePathValue<'a>),
  String(ConcreteStringValue<'a>),
  Action(ConcreteActionValue<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConcreteBooleanValue(bool);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConcreteNumberValue {
  Integer8(i8),
  Integer16(i16),
  Integer32(i32),
  Integer64(i64),
  Integer128(i128),
  Unsigned8(u8),
  Unsigned16(u16),
  Unsigned32(u32),
  Unsigned64(u64),
  Unsigned128(u128),
  Decimal(Decimal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConcretePathValue<'a>(Cow<'a, Path>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConcreteStringValue<'a>(Cow<'a, str>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConcreteActionValue<'a> {
  SetLocation(ConcretePathValue<'a>),
  PushLocation(ConcretePathValue<'a>),
  PopLocation,
}
