use std::pin::Pin;

use crate::http::{Context, Result};

use axum::extract::Request as AxumRequest;
use axum::middleware::{FromFnLayer, Next as AxumNext};
use axum::response::Response as AxumResponse;

pub use sword_macros::{Middleware, middleware};

pub type MiddlewareResult = Result<AxumResponse>;

type AxumMwFn =
    fn(AxumRequest, AxumNext) -> Pin<Box<dyn Future<Output = AxumResponse> + Send + 'static>>;

pub trait MiddlewareLayer {
    fn layer() -> FromFnLayer<AxumMwFn, (), ()>;
}

pub struct NextFunction {
    inner: AxumNext,
}

impl NextFunction {
    pub fn new(next: AxumNext) -> Self {
        Self { inner: next }
    }

    pub async fn run(self, ctx: Context) -> AxumResponse {
        let axum_req = ctx.into_axum_request();
        self.inner.run(axum_req).await
    }
}
