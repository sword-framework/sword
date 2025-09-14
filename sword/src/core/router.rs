use axum::Router as AxumRouter;

use crate::core::state::State;

pub struct _Router {
    _inner: AxumRouter,
}

pub trait RouterProvider {
    fn router(state: State) -> AxumRouter;
}

impl _Router {}
