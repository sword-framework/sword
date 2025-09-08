use std::str::FromStr;

pub mod builder;

use axum::routing::Router;
use axum_responses::http::HttpResponse;
use byte_unit::Byte;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener as Listener;

use crate::{
    core::{
        application::builder::ApplicationBuilder,
        config::{Config, ConfigItem},
    },
    errors::ApplicationError,
};

/// The main application struct that holds the router and configuration.
///
/// `Application` is the core component of the Sword framework that manages
/// the web server, routing, and application configuration. It provides a
/// builder pattern for configuration and methods to run the application.
///
/// # Features
/// - Built on top of Axum for high performance
/// - Configurable via TOML files with environment variable interpolation
/// - Integrated state management for dependency injection
/// - Middleware support for request/response processing
/// - Automatic route registration via controller macros
///
/// # Example
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
/// #[routes]
/// impl HomeController {
///     #[get("/")]
///     async fn index(ctx: Context) -> HttpResult<impl IntoResponse> {
///         let state = ctx.get_state::<AppState>()?;
///         Ok("Hello, World!")
///     }
/// }
///
/// #[main]
/// async fn main() -> Result<(), ApplicationError> {
///     let app_state = AppState::default();
///     
///     let app = Application::builder()?
///         .with_state(app_state)?
///         .with_controller::<HomeController>()
///         .build();
///
///     app.run().await
/// }
/// ```
pub struct Application {
    /// Internal router handling HTTP requests and routing.
    router: Router,
    /// Application configuration loaded from TOML files.
    pub config: Config,
}

/// Configuration structure for the Sword application.
///
/// This struct contains all the configuration options that can be specified
/// in the `config/config.toml` file under the `[application]` section.
///
/// # Configuration File Example
///
/// ```toml,ignore
/// [application]
/// host = "127.0.0.1"
/// port = 3000
/// body_limit = "10MB"
///
/// # Optional: only when multipart feature is enabled
/// allowed_mime_types = ["image/jpeg", "image/png", "application/pdf"]
/// ```
///
/// # Environment Variable Interpolation
///
/// Configuration values support environment variable interpolation:
///
/// ```toml,ignore
/// [application]
/// host = "${HOST:-127.0.0.1}"
/// port = "${PORT:-3000}"
/// ```
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ApplicationConfig {
    /// The hostname or IP address to bind the server to.
    /// Defaults to "127.0.0.1" if not specified.
    pub host: String,
    /// The port number to bind the server to.
    /// Defaults to 3000 if not specified.
    pub port: u16,

    /// Maximum size of request bodies that the server will accept.
    /// Specified as a string with units (e.g., "10MB", "1GB").
    /// Parsed using the byte_unit crate for flexible size specification.
    #[serde(deserialize_with = "deserialize_size")]
    pub body_limit: usize,

    /// List of allowed MIME types for multipart uploads.
    /// Only used when the "multipart" feature is enabled.
    /// Example: ["image/jpeg", "image/png", "application/pdf"]
    #[cfg(feature = "multipart")]
    pub allowed_mime_types: Vec<String>,
}

impl Application {
    /// Creates a new application builder for configuring the application.
    ///
    /// This is the starting point for creating a new Sword application.
    /// The builder pattern allows you to configure various aspects of the
    /// application before building the final `Application` instance.
    ///
    /// # Returns
    ///
    /// Returns `Ok(ApplicationBuilder)` if the configuration can be loaded
    /// successfully, or `Err(ApplicationError)` if there are configuration issues.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The configuration file `config/config.toml` cannot be found
    /// - The configuration file contains invalid TOML syntax
    /// - Environment variable interpolation fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    ///
    /// let app_builder = Application::builder()?;
    /// let app = app_builder
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
    /// # Returns
    ///
    /// Returns `Ok(())` if the server shuts down gracefully, or
    /// `Err(ApplicationError)` if there are issues starting or running the server.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The server fails to bind to the specified address and port
    /// - There are network-related issues during server operation
    /// - The configuration cannot be retrieved from the application state
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    ///
    /// #[main]
    /// async fn main() -> Result<(), ApplicationError> {
    ///     let app = Application::builder()?
    ///         .with_controller::<MyController>()
    ///         .build();
    ///     
    ///     // This will run the server until terminated
    ///     app.run().await
    /// }
    /// ```
    pub async fn run(&self) -> Result<(), ApplicationError> {
        let config = self.config.get::<ApplicationConfig>()?;
        let addr = format!("{}:{}", config.host, config.port);

        let listener = Listener::bind(&addr).await.map_err(|e| ApplicationError::BindFailed {
            address: addr.to_string(),
            source: e,
        })?;

        let router = self.router.clone().fallback(async || {
            HttpResponse::NotFound().message("The requested resource was not found")
        });

        self.display(&config);

        axum::serve(listener, router)
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
    /// # Returns
    ///
    /// A cloned `Router` instance that can be used for testing or integration
    /// with other Axum-based systems.
    ///
    /// # Example
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

    /// Displays the application startup information.
    ///
    /// This method prints the Sword framework logo and configuration information
    /// to the console when the application starts. It shows the host and port
    /// that the server is binding to.
    ///
    /// # Arguments
    ///
    /// * `config` - The application configuration containing host and port information
    ///
    /// # Example Output
    ///
    /// ```text,ignore
    /// ▪──────── ⚔ S W O R D ⚔ ────────▪
    /// Starting Application ...
    /// Host: 127.0.0.1
    /// Port: 3000
    /// ▪──────── ⚔ S W O R D ⚔ ────────▪
    /// ```
    pub fn display(&self, config: &ApplicationConfig) {
        let ascii_logo = "\n▪──────── ⚔ S W O R D ⚔ ────────▪\n";
        println!("{ascii_logo}");
        println!("Starting Application ...");
        println!("Host: {}", config.host);
        println!("Port: {}", config.port);
        println!("{ascii_logo}");
    }
}

/// Implementation of the `ConfigItem` trait for `ApplicationConfig`.
///
/// This implementation allows the application configuration to be automatically
/// loaded from TOML files using the "application" key.
impl ConfigItem for ApplicationConfig {
    /// Returns the TOML key used to identify this configuration section.
    ///
    /// For `ApplicationConfig`, this returns "application", meaning the
    /// configuration should be under the `[application]` section in the TOML file.
    fn toml_key() -> &'static str {
        "application"
    }
}

/// Custom deserializer for size values in configuration.
///
/// This function allows configuration values like "10MB", "1GB", etc. to be
/// automatically converted to byte counts as `usize` values. It uses the
/// `byte_unit` crate to parse human-readable size specifications.
///
/// # Supported Units
///
/// - B (bytes)
/// - KB, MB, GB, TB (decimal)
/// - KiB, MiB, GiB, TiB (binary)
///
/// # Example
///
/// ```toml,ignore
/// [application]
/// body_limit = "10MB"  # Parsed as 10,000,000 bytes
/// # or
/// body_limit = "10MiB" # Parsed as 10,485,760 bytes
/// ```
fn deserialize_size<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Byte::from_str(&s)
        .map(|b| b.as_u64() as usize)
        .map_err(serde::de::Error::custom)
}
