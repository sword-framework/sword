use std::convert::Infallible;

use serde::de::DeserializeOwned;
use tower_layer::Layer;
use tower_service::Service;

use axum::{extract::Request as AxumRequest, response::IntoResponse, routing::Route};

pub(crate) mod cors;
pub(crate) mod logger;

pub trait LayerProvider<L, C>
where
    L: Layer<Route> + Clone + Send + Sync + 'static,
    L::Service: Service<AxumRequest> + Clone + Send + Sync + 'static,
    <L::Service as Service<AxumRequest>>::Response: IntoResponse + 'static,
    <L::Service as Service<AxumRequest>>::Error: Into<Infallible> + 'static,
    <L::Service as Service<AxumRequest>>::Future: Send + 'static,
    C: DeserializeOwned,
{
    fn layer(&self) -> L;
    fn config(&self) -> &C;
}
