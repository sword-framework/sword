pub use crate::__private::AxumNext as Next;
use crate::__private::AxumResponse;

use crate::application::AppState;
use crate::extract::State;
use crate::http::{Request, Result};

pub use sword_macros::middleware;

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

pub trait MiddlewareWithAppState: Send + Sync + 'static {
    fn handle(
        state: State<AppState>,
        req: Request,
        next: Next,
    ) -> impl Future<Output = MiddlewareResult> + Send
    where
        Self: Send + Sync,
        Request: Send,
        Next: Send;
}
