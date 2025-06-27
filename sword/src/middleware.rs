pub use crate::__private::AxumNext as Next;
use crate::__private::AxumResponse;

use crate::extract::State;
use crate::http::{Request, Result};

pub use sword_macros::middleware;

/// MiddlewareResult is the result type returned by middleware handlers.
/// It is a `Result` that contains an axum native Response in both success and error cases.
pub type MiddlewareResult = Result<AxumResponse>;

/// Middleware trait defines the pattern for middleware in the application.
/// It requires the `handle` method to be implemented, which takes a request and a next.
///
/// This implementation allows for asynchronous processing of requests,
/// without using global state of the application. If you need to access shared state,
/// consider using `MiddlewareWithState` instead.
///
/// # Example:
/// ```rust
/// use sword::prelude::*;
///
/// struct MyMiddleware;
///
/// impl Middleware for MyMiddleware {
///    async fn handle(mut req: Request, next: Next) -> MiddlewareResult {
///        req.extensions
///            .insert("Middleware executed successfully".to_string());
///        Ok(next.run(req.into()).await)
///    }
/// }
///
/// #[controller("/api")]
/// struct AppController {}
///
/// #[controller_impl]
/// impl AppController {
///     #[get("/data")]
///     #[middleware(MyMiddleware)]
///     async fn submit_data(req: Request) -> HttpResponse {
///         let mw_message = req.extensions.get::<String>().unwrap();
///         HttpResponse::Ok().message(mw_message)
///     }
/// }
/// ```
pub trait Middleware: Send + Sync + 'static {
    fn handle(req: Request, next: Next) -> impl Future<Output = MiddlewareResult> + Send
    where
        Self: Send + Sync,
        Request: Send,
        Next: Send;
}

/// MiddlewareWithState trait defines the pattern for middleware that requires access to application state.
/// It requires the `handle` method to be implemented, which takes a state, a request, and a next.
/// This allows middleware to access shared state while processing requests.
///
/// # Example:
/// ```rust
/// use serde_json::json;
/// use std::sync::{Arc, OnceLock};
/// use sword::prelude::*;
/// use tokio::sync::RwLock;
///
/// type InMemoryDb = Arc<RwLock<Vec<String>>>;
/// const IN_MEMORY_DB: OnceLock<InMemoryDb> = OnceLock::new();
///
/// fn db() -> Arc<RwLock<Vec<String>>> {
///     IN_MEMORY_DB
///         .get_or_init(|| Arc::new(RwLock::new(Vec::new())))
///         .clone()
/// }
///
/// #[derive(Clone)]
/// struct AppState {
///     db: InMemoryDb,
/// }
///
/// struct MyMiddleware;
///
/// impl MiddlewareWithState<AppState> for MyMiddleware {
///     async fn handle(ctx: State<AppState>, mut req: Request, next: Next) -> MiddlewareResult {
///         let count = ctx.db.read().await.len();
///         req.extensions.insert(count);
///
///         Ok(next.run(req.into()).await)
///     }
/// }
///
/// #[controller("/api")]
/// struct AppController {}
///
/// #[controller_impl]
/// impl AppController {
///     #[get("/data")]
///     #[middleware(MyMiddleware)]
///     async fn submit_data(state: State<AppState>, req: Request) -> HttpResponse {
///         let db = &state.db;
///         let count = req.extensions.get::<usize>().cloned().unwrap_or(0);
///         let message = format!("Current data count: {}", count);
///
///         db.write().await.push(message);
///
///         HttpResponse::Ok().data(json!({
///             "count": count,
///             "current_data": db.read().await.clone(),
///         }))
///     }
/// }
///```
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
