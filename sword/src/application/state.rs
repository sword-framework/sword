use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

#[derive(Clone, Debug)]
pub struct SwordState {
    inner: Arc<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl SwordState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(HashMap::new()),
        }
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref::<T>())
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(self, state: T) -> Self {
        let mut new_map = (*self.inner).clone();
        new_map.insert(TypeId::of::<T>(), Arc::new(state));

        Self {
            inner: Arc::new(new_map),
        }
    }
}

impl Default for SwordState {
    fn default() -> Self {
        Self::new()
    }
}
