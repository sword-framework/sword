use crate::{
    core::State,
    web::{HttpResponse, controller::ControllerExtractor},
};

pub struct ExtractConfig<T>(T);

impl<T> From<ExtractConfig<T>> for ControllerExtractor<T> {
    fn from(extractor: ExtractConfig<T>) -> Self {
        ControllerExtractor::Config(extractor)
    }
}

impl<T> TryFrom<State> for ExtractConfig<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Error = HttpResponse;

    fn try_from(state: State) -> Result<Self, Self::Error> {
        Ok(state.get::<T>().map(|v| ExtractConfig(v))?)
    }
}
