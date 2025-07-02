pub use crate::__private::AxumNext as Next;
use crate::__private::AxumResponse;

use crate::http::{Context, Result};
use std::future::Future;

pub use sword_macros::middleware;

/// MiddlewareResult is the result type returned by middleware handlers.
/// It is a `Result` that contains an axum native Response in both success and error cases.
pub type MiddlewareResult = Result<AxumResponse>;

/// Trait for middlewares without configuration
pub trait Middleware: Send + Sync + 'static {
    fn handle(ctx: Context, next: Next) -> impl Future<Output = MiddlewareResult> + Send;
}

/// Trait for middlewares with configuration
pub trait MiddlewareWithConfig<C>: Send + Sync + 'static {
    fn handle(config: C, req: Context, next: Next)
    -> impl Future<Output = MiddlewareResult> + Send;
}
