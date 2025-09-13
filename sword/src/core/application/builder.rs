use std::{convert::Infallible, time::Duration};

use axum::{
    extract::Request as AxumRequest,
    middleware::from_fn_with_state as mw_with_state,
    response::IntoResponse,
    routing::{Route, Router},
};

use tower::{Layer, Service};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

#[cfg(feature = "cookies")]
use tower_cookies::CookieManagerLayer;

use crate::{
    core::{
        application::{Application, ApplicationConfig},
        config::Config,
        router::RouterProvider,
        state::State,
    },
    errors::{ApplicationError, StateError},
    web::{ContentTypeCheck, ResponsePrettifier},
};

/// Builder for constructing a Sword application with various configuration options.
///
/// `ApplicationBuilder` provides a fluent interface for configuring a Sword application
/// before building the final `Application` instance. It allows you to register
/// controllers, add middleware layers, configure shared state, and set up dependency injection.
///
/// ### Example
///
/// ```rust,ignore
/// use sword::prelude::*;
///
/// #[derive(Default)]
/// struct AppState {
///     counter: std::sync::atomic::AtomicU64,
/// }
///
/// #[controller]
/// struct HomeController;
///
/// let app = Application::builder()?
///     .with_state(AppState::default())?
///     .with_controller::<HomeController>()
///     .with_layer(tower_http::cors::CorsLayer::permissive())
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ApplicationBuilder {
    /// The internal Axum router that handles HTTP requests.
    router: Router,
    /// Shared application state for dependency injection and data sharing.
    state: State,
    /// Application configuration loaded from TOML files.
    pub config: Config,
}

impl ApplicationBuilder {
    /// Creates a new application builder with default configuration.
    ///
    /// This method initializes a new builder with:
    /// - Empty router
    /// - Fresh state container
    /// - Configuration loaded from `config/config.toml`
    ///
    /// ### Returns
    ///
    /// Returns `Ok(ApplicationBuilder)` if initialization succeeds, or
    /// `Err(ApplicationError)` if configuration loading fails.
    ///
    /// ### Errors
    ///
    /// This function will return an error if:
    /// - The configuration file cannot be found or read
    /// - The TOML syntax is invalid
    /// - Environment variable interpolation fails
    pub fn new() -> Result<Self, ApplicationError> {
        let state = State::new();
        let config = Config::new()?;

        state.insert(config.clone()).unwrap();

        let router = Router::new().with_state(state.clone());

        Ok(Self {
            router,
            state,
            config,
        })
    }

    /// Registers a controller in the application.
    ///
    /// This method adds a controller and its routes to the application's router.
    /// Controllers must implement the `RouterProvider` trait, which is typically
    /// done using the `#[controller]` and `#[routes]` macros.
    ///
    /// ### Type Parameters
    ///
    /// * `R` - A type implementing `RouterProvider` that defines the controller's routes
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    ///
    /// #[controller]
    /// struct HomeController;
    ///
    /// #[routes]
    /// impl HomeController {
    ///     #[get("/")]
    ///     async fn index() -> HttpResult<HttpResponse> {
    ///         Ok(HttpResponse::Ok().message("Welcome to the Home Page"))
    ///     }
    /// }
    ///
    /// let app = Application::builder()?
    ///     .with_controller::<HomeController>()
    ///     .build();
    /// ```
    pub fn with_controller<R: RouterProvider>(self) -> Self {
        let controller_router = R::router(self.state.clone());
        let router = self.router.clone().merge(controller_router);

        Self {
            router,
            state: self.state,
            config: self.config,
        }
    }

    /// Registers a middleware layer in the application.
    ///
    /// This method allows you to add Tower-based middleware or other layers
    /// that implement the `Layer` trait. Layers are applied to all routes
    /// in the application and can modify requests and responses.
    ///
    /// ### Arguments
    ///
    /// * `layer` - The middleware layer to add to the application
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use tower_http::cors::CorsLayer;
    /// use tower_http::trace::TraceLayer;
    ///
    /// let app = Application::builder()?
    ///     .with_layer(CorsLayer::permissive())
    ///     .with_layer(TraceLayer::new_for_http())
    ///     .build();
    /// ```
    pub fn with_layer<L>(self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + Sync + 'static,
        L::Service: Service<AxumRequest> + Clone + Send + Sync + 'static,
        <L::Service as Service<AxumRequest>>::Response: IntoResponse + 'static,
        <L::Service as Service<AxumRequest>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<AxumRequest>>::Future: Send + 'static,
    {
        let router = self.router.layer(layer);

        Self {
            router,
            state: self.state,
            config: self.config,
        }
    }

    /// Registers shared state in the application.
    ///
    /// **IMPORTANT**: This method must be called before adding controllers or middleware.
    ///
    /// This method adds shared state that can be accessed by controllers and middleware
    /// throughout the application. The state is automatically wrapped in an `Arc` for
    /// safe sharing across multiple threads.
    ///
    /// State can be retrieved in handlers using `Context::get_state::<T>()`.
    ///
    /// ### Type Parameters
    ///
    /// * `S` - The type of state to store (must implement `Sync + Send + 'static`)
    ///
    /// ### Arguments
    ///
    /// * `state` - The state instance to store in the application
    ///
    /// ### Errors
    ///
    /// This function will return an error if the same state type has already
    /// been registered in the application.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use std::sync::atomic::AtomicU64;
    ///
    /// #[derive(Default)]
    /// struct AppState {
    ///     counter: AtomicU64,
    ///     name: String,
    /// }
    ///
    /// let app_state = AppState {
    ///     counter: AtomicU64::new(0),
    ///     name: "My App".to_string(),
    /// };
    ///
    /// let app = Application::builder()?
    ///     .with_state(app_state)?
    ///     .build();
    /// ```
    pub fn with_state<S: Sync + Send + 'static>(
        self,
        state: S,
    ) -> Result<Self, StateError> {
        self.state.insert(state)?;

        let router = Router::new().with_state(self.state.clone());

        Ok(Self {
            router,
            state: self.state,
            config: self.config,
        })
    }

    /// Registers a Shaku dependency injection module in the application.
    ///
    /// This method integrates Shaku modules for dependency injection, allowing you
    /// to register services and dependencies that can be resolved later using
    /// `Context::di::<ModuleType, InterfaceType>()`.
    ///
    /// Available only when the `shaku-di` feature is enabled.
    ///
    /// ### Type Parameters
    ///
    /// * `M` - The Shaku module type (must implement `Sync + Send + 'static`)
    ///
    /// ### Arguments
    ///
    /// * `module` - The Shaku module instance containing registered services
    ///
    /// ### Returns
    ///
    /// Returns `Ok(Self)` for method chaining, or `Err(StateError)` if the module
    /// type is already registered.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use shaku::{module, Component, Interface};
    /// use std::sync::Arc;
    ///
    /// trait DatabaseService: Interface {
    ///     fn get_connection(&self) -> String;
    /// }
    ///
    /// #[derive(Component)]
    /// #[shaku(interface = DatabaseService)]
    /// struct PostgresService;
    ///
    /// impl DatabaseService for PostgresService {
    ///     fn get_connection(&self) -> String {
    ///         "postgresql://localhost:5432/mydb".to_string()
    ///     }
    /// }
    ///
    /// module! {
    ///     AppModule {
    ///         components = [PostgresService],
    ///         providers = []
    ///     }
    /// }
    ///
    /// let module = AppModule::builder().build();
    ///
    /// let app = Application::builder()?
    ///     .with_shaku_di_module(module)?
    ///     .build();
    /// ```
    #[cfg(feature = "shaku-di")]
    pub fn with_shaku_di_module<M: Sync + Send + 'static>(
        self,
        module: M,
    ) -> Result<Self, StateError> {
        self.with_state(module)
    }

    /// Builds the final application instance.
    ///
    /// This method finalizes the application configuration and creates the
    /// `Application` instance. It applies all configured middleware layers,
    /// sets up request body limits, and prepares the application for running.
    ///
    /// ### Built-in Middleware
    ///
    /// The following middleware is automatically applied:
    /// - Content-Type validation middleware
    /// - Request body size limiting middleware
    /// - Cookie management layer (if `cookies` feature is enabled)
    pub fn build(self) -> Application {
        let mut router = self.router.clone();
        let app_config = self.config.get::<ApplicationConfig>().unwrap();

        router = router
            .layer(mw_with_state(self.state.clone(), ContentTypeCheck::layer))
            .layer(RequestBodyLimitLayer::new(app_config.body_limit));

        if let Some(timeout_secs) = app_config.request_timeout_seconds {
            router =
                router.layer(TimeoutLayer::new(Duration::from_secs(timeout_secs)));
        }

        #[cfg(feature = "cookies")]
        {
            router = router.layer(CookieManagerLayer::new());
        }

        router = router
            .layer(mw_with_state(self.state.clone(), ResponsePrettifier::layer));

        Application {
            router,
            config: self.config,
        }
    }
}
