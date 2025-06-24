use crate::__private::{AxumNext, AxumResponse};
use crate::http::{Context, Result};

pub use sword_macros::{Middleware, middleware};

pub type MiddlewareResult = Result<AxumResponse>;

pub trait MiddlewareHandler: Send + Sync + 'static {
    fn handle(ctx: Context, next: NextFunction) -> impl Future<Output = MiddlewareResult> + Send
    where
        Self: Send + Sync,
        Context: Send,
        NextFunction: Send;
}

pub struct NextFunction {
    inner: AxumNext,
}

unsafe impl Send for NextFunction {}
unsafe impl Sync for NextFunction {}

impl NextFunction {
    pub fn new(next: AxumNext) -> Self {
        Self { inner: next }
    }

    pub async fn run(self, ctx: Context) -> AxumResponse {
        let axum_req = ctx.into_axum_request();
        self.inner.run(axum_req).await
    }
}
