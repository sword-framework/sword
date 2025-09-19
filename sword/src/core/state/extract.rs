use crate::web::ControllerError;

use super::internal::State;
use std::ops::Deref;

#[derive(Clone)]
pub struct ExtractState<T: Send + Sync + 'static + Clone> {
    inner: T,
}

impl<T> TryFrom<&State> for ExtractState<T>
where
    T: Send + Sync + 'static + Clone,
{
    type Error = ControllerError;

    fn try_from(value: &State) -> Result<Self, Self::Error> {
        let inner = value.get::<T>().map_err(|e| {
            eprintln!("{e:?}");
            ControllerError::StateExtractionError(e.to_string())
        })?;

        Ok(ExtractState {
            inner: (*inner).clone(),
        })
    }
}

impl<T: Send + Sync + 'static + Clone> Deref for ExtractState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
