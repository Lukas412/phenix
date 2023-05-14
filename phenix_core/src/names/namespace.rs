use derive_more::Display;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Display)]
pub struct Namespace(String);

impl Namespace {
  pub fn new(value: &str) -> IResult<&str, Self> {}
}

fn is_valid_namespace(value: &str) -> bool {
  
}
