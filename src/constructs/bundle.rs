use std::collections::HashMap;

use super::{namespace::Namespace, pack::Pack};

#[derive(Default)]
pub(crate) struct Bundle<'a> {
    packs: HashMap<Namespace<'a>, Pack>,
}

impl<'a> Bundle<'a> {
    pub(crate) fn add_pack(&mut self, namespace: Namespace<'a>, pack: Pack) {
        self.packs.insert(namespace, pack);
    }
}
