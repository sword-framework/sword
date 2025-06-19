use crate::controller::ControllerKind;
use crate::routing::RouterProvider;

#[derive(Debug, Clone)]
pub struct Application {
    router: axum::Router,
}

impl Application {
    pub fn new() -> Self {
        Application {
            router: axum::Router::new(),
        }
    }

    pub fn add_controller<R: RouterProvider + ControllerKind>(&mut self) -> Self {
        self.router = self.router.clone().merge(R::router());
        self.clone()
    }

    pub async fn run(&self) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("Failed to bind to address");

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
