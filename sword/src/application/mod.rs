use std::convert::Infallible;

use axum::{
    extract::Request as AxumRequest,
    response::IntoResponse,
    routing::{Route, Router},
};

use tokio::net::TcpListener;
use tower_layer::Layer;
use tower_service::Service;

use crate::routing::RouterProvider;

mod state;

pub use state::SwordState;

#[derive(Debug, Clone)]
pub struct Application {
    router: Router,
    state: SwordState,
}

impl Application {
    pub fn builder() -> Self {
        let state = SwordState::new();
        let router = Router::new().with_state(state.clone());

        Self { router, state }
    }

    pub fn controller<R: RouterProvider>(self) -> Self {
        let controller_router = R::router(self.state.clone());
        let router = self.router.clone().merge(controller_router);

        Self {
            router,
            state: self.state,
        }
    }

    pub fn layer<L>(self, layer: L) -> Self
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
        }
    }

    pub fn state<S: Sync + Send + 'static>(self, state: S) -> Self {
        let new_state = self.state.insert(state);
        let router = Router::new().with_state(new_state.clone());

        Self {
            router,
            state: new_state,
        }
    }

    /// Register a Shaku dependency injection module in the application state.
    ///
    /// This is a convenience method that's semantically equivalent to `state()` but provides
    /// better code readability when working with dependency injection modules.
    ///
    /// # Example:
    /// ```rust
    /// use std::sync::Arc;
    /// use shaku::{module, Component, Interface};
    /// use sword::prelude::*;
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
    /// #[controller("/users")]
    /// struct UserController {}
    ///
    /// #[controller_impl]
    /// impl UserController {
    ///     #[get("/")]
    ///     async fn get_users() -> HttpResponse {
    ///         HttpResponse::Ok().data("Users")
    ///     }
    /// }
    ///
    /// let module = AppModule::builder().build();
    ///
    /// let app = Application::builder()
    ///     .di_module(Arc::new(module))
    ///     .controller::<UserController>();
    /// ```
    pub fn di_module<M: Sync + Send + 'static>(self, module: M) -> Self {
        self.state(module)
    }

    pub async fn run(&self, addr: &str) {
        // let config = self.state.get::<Config>().unwrap_or_else(|| {
        //     handle_critical_error(
        //         "Failed to retrieve application configuration",
        //         "Config not found",
        //         Some("sword"),
        //     )
        // });

        // let addr = format!("{}:{}", config.server.host, config.server.port);

        let listener = match TcpListener::bind(&addr).await {
            Ok(listener) => listener,
            Err(e) => panic!("[x] Error - Failed to bind to address {addr}: {e}"),
        };

        let router = self.router.clone();

        axum::serve(listener, router).await.unwrap();
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }
}
