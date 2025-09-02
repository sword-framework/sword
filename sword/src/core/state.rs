use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::errors::StateError;

#[derive(Clone, Debug)]
pub struct State {
    inner: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self) -> Result<Arc<T>, StateError> {
        let map = self.inner.read().map_err(|_| StateError::LockError)?;

        let state_ref = map.get(&TypeId::of::<T>()).ok_or(StateError::TypeNotFound)?;

        state_ref.clone().downcast::<T>().map_err(|_| StateError::DowncastFailed {
            type_name: std::any::type_name::<T>(),
        })
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(&self, state: T) -> Result<(), StateError> {
        let mut map = self.inner.write().map_err(|_| StateError::LockError)?;
        map.insert(TypeId::of::<T>(), Arc::new(state));

        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
