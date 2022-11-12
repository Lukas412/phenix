use rust_lisp::model::Symbol;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Qualifier {
    namespace: Namespace,
    name: Name,
}

impl Qualifier {
    pub(crate) fn new(namespace: Namespace, name: Name) -> Self {
        Self { namespace, name }
    }
}

impl ToString for Qualifier {
    fn to_string(&self) -> String {
        format!("{}::{}", self.namespace.0, self.name.0)
    }
}

impl From<Qualifier> for Symbol {
    fn from(qualifier: Qualifier) -> Self {
        let value = qualifier.to_string();
        Symbol(value)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Name(String);

impl From<&str> for Name {
    fn from(string: &str) -> Self {
        Self(string.to_owned())
    }
}

impl From<String> for Name {
    fn from(string: String) -> Self {
        Self(string)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub(crate) struct Namespace(String);

impl From<&str> for Namespace {
    fn from(string: &str) -> Self {
        Self(string.to_owned())
    }
}

impl From<String> for Namespace {
    fn from(string: String) -> Self {
        Self(string)
    }
}
