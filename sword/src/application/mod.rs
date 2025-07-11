use std::convert::Infallible;

use axum::{
    extract::Request as AxumRequest,
    response::IntoResponse,
    routing::{Route, Router},
};

use serde::Deserialize;
use tokio::net::TcpListener;
use tower_layer::Layer;
use tower_service::Service;

use crate::{
    application::config::{ConfigItem, SwordConfig},
    errors::{ApplicationError, StateError},
    web::RouterProvider,
};

pub mod config;
mod state;

pub use state::SwordState;
pub use sword_macros::config as config_macro;

#[derive(Debug, Clone)]
pub struct Application {
    router: Router,
    state: SwordState,
    config: SwordConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Application {
    pub fn builder() -> Result<Self, ApplicationError> {
        let state = SwordState::new();
        let config = SwordConfig::new()?;

        state.insert(config.clone()).unwrap();

        let router = Router::new().with_state(state.clone());

        Ok(Self {
            router,
            state,
            config,
        })
    }

    /// Register a controller in the application.
    /// This method allows you to add a controller to the application's router.
    pub fn controller<R: RouterProvider>(self) -> Self {
        let controller_router = R::router(self.state.clone());
        let router = self.router.clone().merge(controller_router);

        Self {
            router,
            state: self.state,
            config: self.config,
        }
    }

    /// Register a layer in the application.
    /// This method allows you to add middleware or other layers to the application's router.
    /// This is useful to add tower based middleware or other layers that implement the `Layer` trait.
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
            config: self.config,
        }
    }

    /// Register a state in the application.
    /// This method allows you to add a shared state to the application's router.
    /// The state can be any type that implements `Sync` and `Send`, and is
    /// stored in the application's state.
    ///
    /// It's not necesary to use your state wrapped in `Arc`, as the `SwordState`
    /// already does that for you.
    pub fn state<S: Sync + Send + 'static>(self, state: S) -> Result<Self, StateError> {
        self.state.insert(state)?;

        let router = Router::new().with_state(self.state.clone());

        Ok(Self {
            router,
            state: self.state,
            config: self.config,
        })
    }

    /// Register a dependency injection module in the application.
    /// This method allows you to add a Shaku module to the application's state.
    /// Behind the scenes, it will register the module in the `SwordState` so you can
    /// retrieve it later using the `get_dependency` method.
    pub fn di_module<M: Sync + Send + 'static>(self, module: M) -> Result<Self, StateError> {
        self.state(module)
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        let server_conf = self.config.get::<ServerConfig>()?;
        let addr = format!("{}:{}", server_conf.host, server_conf.port);

        let listener =
            TcpListener::bind(&addr)
                .await
                .map_err(|e| ApplicationError::BindFailed {
                    address: addr.to_string(),
                    source: e,
                })?;

        let router = self.router.clone();

        println!("Starting server on {addr}");

        axum::serve(listener, router)
            .await
            .map_err(|e| ApplicationError::ServerError { source: e })?;

        Ok(())
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }
}

impl ConfigItem for ServerConfig {
    fn toml_key() -> &'static str {
        "application"
    }
}
