use crate::{
    core::State,
    web::{HttpResponse, controller::ControllerExtractor},
};

pub struct ExtractState<T>(T);

impl<T> From<ExtractState<T>> for ControllerExtractor<T> {
    fn from(extractor: ExtractState<T>) -> Self {
        ControllerExtractor::State(extractor)
    }
}

impl<T> TryFrom<State> for ExtractState<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Error = HttpResponse;

    fn try_from(state: State) -> Result<Self, Self::Error> {
        Ok(state.get::<T>().map(|v| ExtractState(v))?)
    }
}
