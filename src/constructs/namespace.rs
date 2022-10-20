pub(crate) type Name<'a> = &'a str;

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Namespace<'a> {
    parts: Vec<Name<'a>>,
}

pub(crate) struct Qualifier<'a> {
    namespace: Namespace<'a>,
    name: Name<'a>,
}

impl<'a> Namespace<'a> {
    pub(crate) fn new(parts: Vec<Name>) -> Self {
        Self { parts }
    }
}
