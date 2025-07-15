use std::convert::Infallible;

use axum::{
    extract::Request as AxumRequest,
    middleware::Next,
    response::IntoResponse,
    routing::{Route, Router},
};

use axum_responses::http::HttpResponse;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_layer::Layer;
use tower_service::Service;

use crate::{
    __private::mw_with_state,
    application::config::{ConfigItem, SwordConfig},
    errors::{ApplicationError, RequestError, StateError},
    next,
    web::{Context, MiddlewareResult, RouterProvider},
};

pub mod config;
mod state;

pub use state::SwordState;
pub use sword_macros::config as config_macro;

const NO_BODY_METHODS: [&str; 4] = ["GET", "DELETE", "HEAD", "OPTIONS"];

#[derive(Debug, Clone)]
pub struct Application {
    router: Router,
    state: SwordState,
    pub config: SwordConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub host: String,
    pub port: u16,
}

impl Application {
    pub fn builder() -> Result<Self, ApplicationError> {
        let state = SwordState::new();
        let config = SwordConfig::new()?;

        state.insert(config.clone()).unwrap();

        let router = Router::new()
            .layer(mw_with_state(
                state.clone(),
                Application::content_type_check,
            ))
            .with_state(state.clone());

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
        let server_conf = self.config.get::<ApplicationConfig>()?;
        let addr = format!("{}:{}", server_conf.host, server_conf.port);

        let listener =
            TcpListener::bind(&addr)
                .await
                .map_err(|e| ApplicationError::BindFailed {
                    address: addr.to_string(),
                    source: e,
                })?;

        let mut router = self.router.clone().fallback(async || {
            HttpResponse::NotFound().message("The requested resource was not found")
        });

        router = router.layer(mw_with_state(
            self.state.clone(),
            Application::content_type_check,
        ));

        let ascii_logo = "\n▪──────── ⚔ S W O R D ⚔ ────────▪\n";

        println!("{ascii_logo}");
        println!("Starting Application ...");
        println!("Host: {}", server_conf.host);
        println!("Port: {}", server_conf.port);
        println!("{ascii_logo}");

        axum::serve(listener, router)
            .await
            .map_err(|e| ApplicationError::ServerError { source: e })?;

        Ok(())
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    async fn content_type_check(ctx: Context, next: Next) -> MiddlewareResult {
        let method = ctx.method().as_str();
        let content_type = ctx.header("Content-Type").unwrap_or_default();

        if NO_BODY_METHODS.contains(&method) {
            return next!(ctx, next);
        }

        if content_type != "application/json" && !content_type.contains("multipart/form-data") {
            return Err(RequestError::InvalidContentType(
                "Only application/json and multipart/form-data content types are supported.",
            )
            .into());
        }

        next!(ctx, next)
    }
}

impl ConfigItem for ApplicationConfig {
    fn toml_key() -> &'static str {
        "application"
    }
}

// mod utils {
//     use std::str::FromStr;

//     use byte_unit::Byte;
//     use regex::Regex;
//     use serde::{Deserialize, Deserializer};

//     pub fn deserialize_body_limit<'de, D>(deserializer: D) -> Result<usize, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let body_limit: String = Deserialize::deserialize(deserializer)?;
//         let regex = Regex::new(r"(?i)^\s*(\d+(?:\.\d+)?)\s*(b|kb|mb|gb)?\s*$").map_err(|_| {
//             serde::de::Error::custom("Invalid body limit format. Expected a number followed by an optional unit (b, kb, mb, gb).")
//         })?;

//         if !regex.is_match(&body_limit) {
//             return Err(serde::de::Error::custom(
//                 "Invalid body limit format. Expected a number followed by an optional unit (b, kb, mb, gb).",
//             ));
//         }

//         let bytes = Byte::from_str(&body_limit)
//             .map_err(|_| serde::de::Error::custom("Failed to parse body limit"))?;

//         println!("Parsed body limit: {}", bytes);

//         Ok(bytes.as_u64() as usize)
//     }
// }
