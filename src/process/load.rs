use crate::constructs::{Bundle, Namespace, ValueType};
use crate::Pack;
use rust_lisp::model::Value;
use std::io;

pub(crate) fn load_packs<'a>() -> Result<Bundle<'a>, LoadPacksError> {
    let mut bundle = Bundle::default();

    let namespace = Namespace::new(vec!["test", "pack"]);
    let pack = Pack::new(
        Value::String("Wow! This is working!".to_owned()),
        ValueType::String,
    );
    bundle.add_pack(namespace, pack);

    Ok(bundle)
}

pub(crate) enum LoadPacksError {
    IoError(io::Error),
}
