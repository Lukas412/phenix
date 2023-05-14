use derive_more::{Display, Error};
use nom::bytes::complete::take_while1;
use nom::error::{ErrorKind, ParseError};
use nom::{Finish, IResult, InputTakeAtPosition};
use std::str::FromStr;

pub struct Name(String);

impl FromStr for Name {
  type Err = ();

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let (_, name) = take_name(string).finish()?;
    Ok(Name(name.to_owned()))
  }
}

pub(crate) fn take_name<Input>(input: Input) -> IResult<Input, Input, ParseNameError<Input>>
where
  Input: InputTakeAtPosition<Item = char>,
{
  take_while1(char::is_alphanumeric)(input)
}

#[derive(Debug, Display, Error)]
pub struct ParseNameError<Input> {
  value: Input,
}

impl<Input> ParseError<Input> for ParseNameError<Input> {
  fn from_error_kind(input: Input, _kind: ErrorKind) -> Self {
    todo!()
  }

  fn append(input: Input, kind: ErrorKind, other: Self) -> Self {
    todo!()
  }
}
