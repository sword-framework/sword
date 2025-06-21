mod config;

use tokio::net::TcpListener;

use crate::{application::config::Config, routing::RouterProvider, utils::handle_critical_error};

#[derive(Debug, Clone)]
pub struct Application {
    router: axum::Router,
    config: config::Config,
}

impl Application {
    pub fn new() -> Self {
        let config = match Config::new() {
            Ok(cfg) => cfg,
            Err(e) => handle_critical_error("Failed to load configuration", e, Some("config-rs")),
        };

        Application {
            router: axum::Router::new(),
            config,
        }
    }

    pub fn add_controller<R: RouterProvider>(&mut self) -> Self {
        self.router = self.router.clone().merge(R::router());
        self.clone()
    }

    pub async fn run(&self) {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);

        let listener = match TcpListener::bind(&addr).await {
            Ok(listener) => listener,
            Err(e) => handle_critical_error("Failed to bind to address", e, Some("tokio-rs")),
        };

        axum::serve(listener, self.router.clone()).await.unwrap();
    }

    pub fn router(&self) -> axum::Router {
        self.router.clone()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
