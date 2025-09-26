use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::errors::StateError;

/// Application state container for type-safe dependency injection and data sharing.
///
/// `State` provides a thread-safe way to store and retrieve shared data across
/// the entire application. It uses `TypeId` as keys to ensure type safety and
/// prevents type confusion. State is automatically managed by the framework
/// and can be accessed through the `Context` in route handlers and middleware.
/// ```
#[derive(Clone, Debug)]
pub struct State {
    inner: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get<T>(&self) -> Result<T, StateError>
    where
        T: Clone + Send + Sync + 'static,
    {
        let map = self.inner.read().map_err(|_| StateError::LockError)?;

        let state_ref = map
            .get(&TypeId::of::<T>())
            .ok_or(StateError::TypeNotFound)?;

        state_ref
            .downcast_ref::<T>()
            .cloned()
            .ok_or(StateError::TypeNotFound)
    }

    pub fn borrow<T>(&self) -> Result<Arc<T>, StateError>
    where
        T: Send + Sync + 'static,
    {
        let map = self.inner.read().map_err(|_| StateError::LockError)?;

        let state_ref = map
            .get(&TypeId::of::<T>())
            .ok_or(StateError::TypeNotFound)?;

        state_ref
            .clone()
            .downcast::<T>()
            .map_err(|_| StateError::TypeNotFound)
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(
        &self,
        state: T,
    ) -> Result<(), StateError> {
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
