pub use crate::__private::AxumNext as Next;
use crate::__private::AxumResponse;

use crate::http::{Context, Result};
use std::future::Future;

pub use sword_macros::middleware;

/// MiddlewareResult is the result type returned by middleware handlers.
/// It is a `Result` that contains an axum native Response in both success and error cases.
pub type MiddlewareResult = Result<AxumResponse>;

/// Trait for build middlewares that can be used in the application.
pub trait Middleware: Send + Sync + 'static {
    fn handle(ctx: Context, next: Next) -> impl Future<Output = MiddlewareResult> + Send;
}

/// Trait for build middlewares that can be used in the application with a generic
/// configuration parameters, like a secret key, vector of roles, Custom structs and more.
pub trait MiddlewareWithConfig<C>: Send + Sync + 'static {
    fn handle(config: C, req: Context, next: Next)
    -> impl Future<Output = MiddlewareResult> + Send;
}
