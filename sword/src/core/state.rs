use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::errors::StateError;

/// Application state container for type-safe dependency injection and data sharing.
///
/// `State` provides a thread-safe way to store and retrieve shared data across
/// the entire application. It uses `TypeId` as keys to ensure type safety and
/// prevents type confusion. State is automatically managed by the framework
/// and can be accessed through the `Context` in route handlers and middleware.
///
/// ### Example
///
/// ```rust,ignore
/// use sword::prelude::*;
/// use std::sync::atomic::{AtomicU64, Ordering};
///
/// #[derive(Default)]
/// struct AppState {
///     counter: AtomicU64,
///     name: String,
/// }
///
/// // Register state during application building
/// let app = Application::builder()?
///     .with_state(AppState {
///         counter: AtomicU64::new(0),
///         name: "My App".to_string(),
///     })?
///     .build();
///
/// // Access state in route handlers
/// #[get("/count")]
/// async fn get_count(ctx: Context) -> HttpResult<HttpResponse> {
///     let state = ctx.get_state::<AppState>()?;
///     let count = state.counter.load(Ordering::SeqCst);
///
///     Ok(HttpResponse::Ok().data(format!("Count: {}", count)))
/// }
/// ```
#[derive(Clone, Debug)]
pub struct State {
    inner: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self) -> Result<Arc<T>, StateError> {
        let map = self.inner.read().map_err(|_| StateError::LockError)?;

        let state_ref = map.get(&TypeId::of::<T>()).ok_or(StateError::TypeNotFound)?;

        state_ref.clone().downcast::<T>().map_err(|_| StateError::DowncastFailed {
            type_name: std::any::type_name::<T>(),
        })
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(&self, state: T) -> Result<(), StateError> {
        let mut map = self.inner.write().map_err(|_| StateError::LockError)?;
        map.insert(TypeId::of::<T>(), Arc::new(state));

        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
