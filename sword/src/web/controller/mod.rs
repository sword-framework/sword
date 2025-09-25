use crate::core::State as SwordState;
use axum::Router as AxumRouter;
use thiserror::Error;

mod extractors {
    pub mod config;
    pub mod state;

    pub use config::ExtractConfig;
    pub use state::ExtractState;
}

pub use extractors::*;

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("State extraction failed: {0}")]
    StateExtractionError(String),
}

pub trait Controller: ControllerBuilder {
    fn router(state: SwordState) -> AxumRouter;
}

pub trait ControllerBuilder {
    fn base_path() -> &'static str;
    fn apply_controller_middlewares(
        router: AxumRouter,
        app_state: SwordState,
    ) -> AxumRouter;

    fn build(state: SwordState) -> Result<Self, ControllerError>
    where
        Self: Sized;
}

pub enum ControllerExtractor<T> {
    Config(ExtractConfig<T>),
    State(ExtractState<T>),
}
