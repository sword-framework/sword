pub mod builder;
mod config;

pub use config::ApplicationConfig;

use axum::routing::Router;
use axum_responses::http::HttpResponse;
use tokio::net::TcpListener as Listener;

use crate::{
    core::{application::builder::ApplicationBuilder, config::Config},
    errors::ApplicationError,
};

/// The main application struct that holds the router and configuration.
///
/// `Application` is the core component of the Sword framework that manages
/// the web server, routing, and application configuration. It provides a
/// builder pattern for configuration and methods to run the application.
pub struct Application {
    router: Router,
    pub config: Config,
}

impl Application {
    /// Creates a new application builder for configuring the application.
    ///
    /// This is the starting point for creating a new Sword application.
    /// The builder pattern allows you to configure various aspects of the
    /// application before building the final `Application` instance.
    ///
    /// ### Returns
    ///
    /// Returns `Ok(ApplicationBuilder)` if the configuration can be loaded
    /// successfully, or `Err(ApplicationError)` if there are configuration issues.
    ///
    /// ### Errors
    ///
    /// This function will return an error if:
    /// - The configuration file `config/config.toml` cannot be found
    /// - The configuration file contains invalid TOML syntax
    /// - Environment variable interpolation fails
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    ///
    /// let app = Application::builder()?
    ///     .with_controller::<MyController>()
    ///     .build();
    /// ```
    pub fn builder() -> Result<ApplicationBuilder, ApplicationError> {
        ApplicationBuilder::new()
    }

    /// Runs the application server.
    ///
    /// This method starts the web server and begins listening for incoming
    /// HTTP requests. It will bind to the host and port specified in the
    /// application configuration and run until the process is terminated.
    ///
    /// If graceful shutdown is enabled in the configuration, it will handle
    /// termination signals and allow ongoing requests to complete before shutting down.
    ///
    /// ### Returns
    ///
    /// Returns `Ok(())` if the server shuts down gracefully, or
    /// `Err(ApplicationError)` if there are issues starting or running the server.
    ///
    /// ### Errors
    ///
    /// This function will return an error if:
    /// - The server fails to bind to the specified address and port
    /// - There are network-related issues during server operation
    /// - The configuration cannot be retrieved from the application state
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    ///
    /// #[sword::main]
    /// async fn main() {
    ///     let app = Application::builder()?
    ///         .with_controller::<MyController>()
    ///         .build();
    ///     
    ///     app.run().await?;
    /// }
    /// ```
    pub async fn run(&self) -> Result<(), ApplicationError> {
        if self.config.get::<ApplicationConfig>()?.graceful_shutdown {
            return self
                .run_with_graceful_shutdown(Self::graceful_signal())
                .await;
        }

        let listener = self.pre_run().await?;

        let router = self.router.clone().fallback(async || {
            HttpResponse::NotFound().message("The requested resource was not found")
        });

        axum::serve(listener, router)
            .await
            .map_err(|e| ApplicationError::ServerError { source: e })?;

        Ok(())
    }

    /// Runs the application server with graceful shutdown support.
    /// Is similar to `run` but accepts a shutdown signal.
    ///
    /// See [Axum's docs](https://docs.rs/axum/latest/axum/serve/struct.WithGracefulShutdown.html)
    /// to learn more about graceful shutdown.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use tokio::signal;
    ///
    /// #[controller("/admin")]
    /// struct AdminController {}
    ///
    /// #[routes]
    /// impl AdminController {
    ///     #[get("/")]
    ///     async fn get_admin_data() -> HttpResponse {
    ///         HttpResponse::Ok()
    ///     }
    /// }
    ///
    /// #[sword::main]
    /// async fn main() {
    ///     let app = Application::builder()?
    ///         .with_controller::<AppController>()
    ///         .with_controller::<AdminController>()
    ///         .build();
    ///
    ///     app.run_with_graceful_shutdown(shutdown_signal()).await?;
    /// }
    ///
    /// async fn shutdown_signal() {
    ///     let ctrl_c = async {
    ///         signal::ctrl_c()
    ///             .await
    ///             .expect("failed to install Ctrl+C handler");
    ///     };
    ///
    ///     #[cfg(unix)]
    ///     let terminate = async {
    ///         signal::unix::signal(signal::unix::SignalKind::terminate())
    ///             .expect("failed to install signal handler")
    ///             .recv()
    ///             .await;
    ///     };
    ///
    ///     #[cfg(not(unix))]
    ///     let terminate = std::future::pending::<()>();
    ///
    ///     tokio::select! {
    ///         _ = ctrl_c => {},
    ///         _ = terminate => {},
    ///     }
    /// }
    pub async fn run_with_graceful_shutdown<F>(
        &self,
        signal: F,
    ) -> Result<(), ApplicationError>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let listener = self.pre_run().await?;

        let router = self.router.clone().fallback(async || {
            HttpResponse::NotFound().message("The requested resource was not found")
        });

        axum::serve(listener, router)
            .with_graceful_shutdown(signal)
            .await
            .map_err(|e| ApplicationError::ServerError { source: e })?;

        Ok(())
    }

    /// Returns a clone of the internal Axum router.
    ///
    /// This method provides access to the underlying Axum router for advanced
    /// use cases where direct router manipulation is needed. Most applications
    /// should not need to use this method directly.
    ///
    /// ### Returns
    ///
    /// A cloned `Router` instance that can be used for testing or integration
    /// with other Axum-based systems.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    ///
    /// let app = Application::builder()?
    ///     .with_controller::<MyController>()
    ///     .build();
    ///
    /// let router = app.router();
    /// // Use router for testing or other purposes
    /// ```
    pub fn router(&self) -> Router {
        self.router.clone()
    }

    async fn pre_run(&self) -> Result<Listener, ApplicationError> {
        let config = self.config.get::<ApplicationConfig>()?;
        let addr = format!("{}:{}", config.host, config.port);

        let listener = Listener::bind(&addr).await.map_err(|e| {
            ApplicationError::BindFailed {
                address: addr.to_string(),
                source: e,
            }
        })?;

        self.display(&config);

        Ok(listener)
    }

    fn display(&self, config: &ApplicationConfig) {
        let ascii_logo_top = "\n▪──────── ⚔ S W O R D ⚔ ────────▪\n";
        let ascii_logo_bottom = "\n▪──────── ⚔ ───────── ⚔ ────────▪\n";
        println!("{ascii_logo_top}");
        println!("Host: {}", config.host);
        println!("Port: {}", config.port);
        println!("Request Body Limit: {}", config.body_limit.raw);
        println!(
            "Request timeout: {}",
            if let Some(timeout) = config.request_timeout_seconds {
                format!("{} seconds", timeout)
            } else {
                "disabled".to_string()
            }
        );

        println!(
            "Graceful shutdown: {}",
            if config.graceful_shutdown {
                "enabled"
            } else {
                "disabled"
            }
        );

        println!("{ascii_logo_bottom}");
    }

    async fn graceful_signal() {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                println!(" Shutdown signal received, starting graceful shutdown...");
            },
            _ = terminate => {
                println!(" Shutdown signal received, starting graceful shutdown...");
            },
        }
    }
}
