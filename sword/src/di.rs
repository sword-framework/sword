use crate::{application::SwordState, http::HttpResponse};

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use shaku::{HasComponent, Interface, Module};
use std::sync::Arc;

/// Extractor for dependency injection using Shaku modules stored in SwordState
///
/// This extractor allows you to inject dependencies from a Shaku module that is stored
/// in the application state. It works by extracting the module from SwordState and then
/// resolving the requested interface.
///
/// # Example:
/// ```rust
/// use std::sync::Arc;
/// use shaku::{module, Component, Interface};
/// use sword::{di::Inject, prelude::*};
///
/// trait Logger: Interface {
///     fn log(&self, message: &str);
/// }
///
/// #[derive(Component)]
/// #[shaku(interface = Logger)]
/// struct ConsoleLogger;
///
/// impl Logger for ConsoleLogger {
///     fn log(&self, message: &str) {
///         println!("Log: {}", message);
///     }
/// }
///
/// module! {
///     AppModule {
///         components = [ConsoleLogger],
///         providers = []
///     }
/// }
///
/// #[controller("/api")]
/// struct AppController {}
///
/// #[controller_impl]
/// impl AppController {
///     #[get("/users")]
///     async fn get_users(logger: Inject<AppModule, dyn Logger>) -> HttpResponse {
///         logger.log("Fetching users");
///         HttpResponse::Ok().data("Users fetched successfully")
///     }
/// }
/// ```
pub struct Inject<M: Module + ?Sized, I: Interface + ?Sized> {
    interface: Arc<I>,
    _module: std::marker::PhantomData<M>,
}

#[allow(clippy::should_implement_trait)]
impl<M: Module + ?Sized, I: Interface + ?Sized> Inject<M, I> {
    pub fn as_ref(&self) -> &I {
        &self.interface
    }
}

impl<M: Module + ?Sized, I: Interface + ?Sized> std::ops::Deref for Inject<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.interface
    }
}

impl<S, M, I> FromRequestParts<S> for Inject<M, I>
where
    M: Module + HasComponent<I> + Send + Sync + 'static,
    I: Interface + ?Sized + 'static,
    Arc<M>: Send + Sync + 'static,
    SwordState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HttpResponse;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = SwordState::from_ref(state);

        let module = app_state.get::<Arc<M>>().ok_or_else(|| {
            HttpResponse::InternalServerError()
                .message("Dependency injection module not found in application state")
        })?;

        let interface = module.resolve();

        Ok(Self {
            interface,
            _module: std::marker::PhantomData,
        })
    }
}
