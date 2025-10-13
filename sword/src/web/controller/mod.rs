use crate::{core::State as SwordState, errors::DependencyInjectionError};
use axum::Router as AxumRouter;

pub trait Controller: ControllerBuilder {
    fn router(state: SwordState) -> AxumRouter;
}

pub trait ControllerBuilder {
    fn base_path() -> &'static str;

    fn apply_controller_middlewares(
        router: AxumRouter,
        app_state: SwordState,
    ) -> AxumRouter;

    fn build(state: SwordState) -> Result<Self, DependencyInjectionError>
    where
        Self: Sized;
}
