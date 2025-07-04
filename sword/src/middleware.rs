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

/// A macro to simplify the next middleware call in the middleware chain.
/// It takes the current context and the next middleware in the chain,
/// and returns a `Result` with the response of the next middleware.
/// This macro is used to avoid boilerplate code in middleware implementations.
/// It is used in the `handle` method of the `Middleware` trait.
///
/// # Example usage:
/// ```rust
/// use sword::prelude::*;
///
/// struct MyMiddleware;
///
/// impl Middleware for MyMiddleware {
///     async fn handle(ctx: Context, next: Next) -> MiddlewareResult {
///         next!(ctx, next)
///     }
/// }
#[macro_export]
macro_rules! next {
    ($ctx:expr, $next:expr) => {
        Ok($next.run($ctx.try_into()?).await)
    };
}
