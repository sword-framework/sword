use crate::__private::AxumResponse;
use crate::http::{Request, Result};
use crate::prelude::State;

pub use axum::middleware::Next;
pub use sword_macros::{Middleware, middleware};

pub type MiddlewareResult = Result<AxumResponse>;

pub trait Middleware: Send + Sync + 'static {
    fn handle(req: Request, next: Next) -> impl Future<Output = MiddlewareResult> + Send
    where
        Self: Send + Sync,
        Request: Send,
        Next: Send;
}

pub trait MiddlewareWithState<T>: Send + Sync + 'static {
    fn handle(
        state: State<T>,
        req: Request,
        next: Next,
    ) -> impl Future<Output = MiddlewareResult> + Send
    where
        Self: Send + Sync,
        Request: Send,
        Next: Send;
}
