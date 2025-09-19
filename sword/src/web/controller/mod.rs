use crate::core::State as SwordState;
use axum::Router as AxumRouter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("State extraction failed: {0}")]
    StateExtractionError(String),
}

pub trait Controller {
    fn router(state: SwordState) -> AxumRouter;
    fn build_from_state(state: SwordState) -> Result<Self, ControllerError>
    where
        Self: Sized;
}
