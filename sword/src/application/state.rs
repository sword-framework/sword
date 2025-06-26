use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::application::Config;

#[derive(Clone, Debug)]
pub struct AppState {
    inner: Arc<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::new().unwrap_or_else(|_| {
            eprintln!("Failed to load configuration, using default values.");
            Config::default()
        });

        let base_state = HashMap::from([(
            TypeId::of::<Config>(),
            Arc::new(config) as Arc<dyn Any + Send + Sync>,
        )]);

        Self {
            inner: Arc::new(base_state),
        }
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref::<T>())
    }

    pub fn get_cloned<T: Clone + Send + Sync + 'static>(&self) -> Option<T> {
        self.get::<T>().cloned()
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(self, state: T) -> Self {
        let mut new_map = (*self.inner).clone();
        new_map.insert(TypeId::of::<T>(), Arc::new(state));

        Self {
            inner: Arc::new(new_map),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
