use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rust_lisp::model::{Env, Value};

use super::{
    namespace::Namespace,
    pack::{Pack, PackError},
};

#[derive(Default)]
pub(crate) struct Bundle {
    packs: HashMap<Namespace, Pack>,
}

impl Bundle {
    pub(crate) fn add_pack(&mut self, namespace: Namespace, pack: Pack) {
        self.packs.insert(namespace, pack);
    }

    pub(crate) fn eval(
        &self,
        env: Rc<RefCell<Env>>,
        creation: super::Creation,
    ) -> Result<Value, BundleError> {
        let env = creation.extend_env(env);
        let pack = self.get_pack(creation.get_namespace())?;
        pack.eval(env).map_err(BundleError::from)
    }

    pub(crate) fn get_pack(&self, namespace: &Namespace) -> Result<&Pack, BundleError> {
        self.packs
            .get(namespace)
            .ok_or_else(|| BundleError::new_pack_not_found_error(namespace.clone()))
    }
}

#[derive(Debug)]
pub(crate) enum BundleError {
    PackError(PackError),
    PackNotFound { namespace: Namespace },
}

impl BundleError {
    fn new_pack_not_found_error(namespace: Namespace) -> Self {
        Self::PackNotFound { namespace }
    }
}

impl From<PackError> for BundleError {
    fn from(error: PackError) -> Self {
        BundleError::PackError(error)
    }
}
