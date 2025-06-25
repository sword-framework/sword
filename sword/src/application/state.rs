use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
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

    pub fn insert<T: Send + Sync + 'static>(self, state: T) -> Self {
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

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_responses::http::HttpResponse;

pub struct State<T>(pub T);

impl<S, T> FromRequestParts<S> for State<T>
where
    T: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HttpResponse;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let inner_state = app_state
            .get::<T>()
            .ok_or_else(|| HttpResponse::InternalServerError().message("State not found"))?;

        Ok(Self(inner_state.clone()))
    }
}

impl<T> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
