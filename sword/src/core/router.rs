use axum::Router as AxumRouter;

use crate::core::state::State;

pub struct Router {
    _inner: AxumRouter,
}

pub trait RouterProvider {
    fn router(state: State) -> AxumRouter;
}

impl Router {}
