mod config;
pub mod state;

use axum::routing::Router;
use tokio::net::TcpListener;

use crate::{
    application::{config::Config, state::AppState},
    routing::RouterProvider,
    utils::handle_critical_error,
};

#[derive(Debug, Clone)]
pub struct Application {
    router: Router,
    state: AppState,
    config: Config,
}

impl Application {
    pub fn new() -> Application {
        let config = match Config::new() {
            Ok(cfg) => cfg,
            Err(e) => handle_critical_error("Failed to load configuration", e, Some("config-rs")),
        };

        let state = AppState::new(config.clone());
        let router = axum::Router::new().with_state(state.clone());

        Application {
            router,
            state,
            config,
        }
    }

    pub fn add_controller<R: RouterProvider>(&mut self) -> Application {
        let controller_router = R::router(self.state.clone());
        self.router = self.router.clone().merge(controller_router);
        self.clone()
    }

    pub async fn run(&self) {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);

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

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
