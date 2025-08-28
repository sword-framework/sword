use std::{convert::Infallible, str::FromStr};

use axum::{
    extract::Request as AxumRequest,
    response::IntoResponse,
    routing::{Route, Router},
};

use axum_responses::http::HttpResponse;
use byte_unit::Byte;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener as Listener;
use tower_http::limit::RequestBodyLimitLayer;
use tower_layer::Layer;
use tower_service::Service;

use crate::{
    __private::mw_with_state,
    application::config::{ConfigItem, SwordConfig},
    errors::{ApplicationError, StateError},
    web::{RouterProvider, content_type_check},
};

pub mod config;
mod state;

pub use state::SwordState;
pub use sword_macros::config as config_macro;

#[derive(Debug, Clone)]
pub struct Application {
    router: Router,
    state: SwordState,
    pub config: SwordConfig,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ApplicationConfig {
    pub host: String,
    pub port: u16,

    #[serde(deserialize_with = "deserialize_size")]
    pub body_limit: usize,

    #[cfg(feature = "multipart")]
    pub allowed_mime_types: Vec<String>,
}

impl Application {
    pub fn builder() -> Result<Self, ApplicationError> {
        let state = SwordState::new();
        let config = SwordConfig::new()?;

        let app_config = config.get::<ApplicationConfig>()?;

        state.insert(config.clone()).unwrap();

        let mut router = Router::new().with_state(state.clone());

        if cfg!(test) {
            router = router.layer(mw_with_state(state.clone(), content_type_check));
            router = router.layer(RequestBodyLimitLayer::new(app_config.body_limit));
        }

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
        let config = self.config.get::<ApplicationConfig>()?;
        let addr = format!("{}:{}", config.host, config.port);

        let listener = Listener::bind(&addr)
            .await
            .map_err(|e| ApplicationError::BindFailed {
                address: addr.to_string(),
                source: e,
            })?;

        let router = self
            .router
            .clone()
            .layer(mw_with_state(self.state.clone(), content_type_check))
            .layer(RequestBodyLimitLayer::new(config.body_limit))
            .fallback(async || {
                HttpResponse::NotFound().message("The requested resource was not found")
            });

        self.display(&config);

        axum::serve(listener, router)
            .await
            .map_err(|e| ApplicationError::ServerError { source: e })?;

        Ok(())
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub fn display(&self, config: &ApplicationConfig) {
        let ascii_logo = "\n▪──────── ⚔ S W O R D ⚔ ────────▪\n";
        println!("{ascii_logo}");
        println!("Starting Application ...");
        println!("Host: {}", config.host);
        println!("Port: {}", config.port);
        println!("{ascii_logo}");
    }
}

impl ConfigItem for ApplicationConfig {
    fn toml_key() -> &'static str {
        "application"
    }
}

fn deserialize_size<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Byte::from_str(&s)
        .map(|b| b.as_u64() as usize)
        .map_err(serde::de::Error::custom)
}
