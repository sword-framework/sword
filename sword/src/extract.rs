use crate::{application::AppState, http::HttpResponse};

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use std::ops::Deref;

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
