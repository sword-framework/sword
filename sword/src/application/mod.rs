use std::convert::Infallible;

use axum::{
    extract::Request as AxumRequest,
    response::IntoResponse,
    routing::{Route, Router},
};

use tokio::net::TcpListener;
use tower_layer::Layer;
use tower_service::Service;

use crate::{routing::RouterProvider, utils::handle_critical_error};

mod config;
mod state;

pub use config::Config;
pub use state::AppState;

#[derive(Debug, Clone)]
pub struct Application {
    router: Router,
    state: AppState,
}

impl Application {
    pub fn builder() -> Self {
        let state = AppState::new();
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

    pub async fn run(&self) {
        let config = self.state.get::<Config>().unwrap_or_else(|| {
            handle_critical_error(
                "Failed to retrieve application configuration",
                "Config not found",
                Some("sword"),
            )
        });

        let addr = format!("{}:{}", config.server.host, config.server.port);

        let listener = match TcpListener::bind(&addr).await {
            Ok(listener) => listener,
            Err(e) => handle_critical_error("Failed to bind to address", e, Some("tokio-rs")),
        };

        let router = self.router.clone();
        axum::serve(listener, router).await.unwrap();
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }
}
