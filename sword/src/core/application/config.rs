use serde::{Deserialize, Serialize};

use crate::core::ConfigItem;
use crate::core::utils::deserialize_size;

/// Configuration structure for the Sword application.
///
/// This struct contains all the configuration options that can be specified
/// in the `config/config.toml` file under the `[application]` section.
///
/// ### Configuration File Example
///
/// ```toml,ignore
/// [application]
/// host = "127.0.0.1"
/// port = 3000
/// body_limit = "10MB"
///
/// ### Optional: only when multipart feature is enabled
/// allowed_mime_types = ["image/jpeg", "image/png", "application/pdf"]
/// ```
///
/// ### Environment Variable Interpolation
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
    /// Defaults to "0.0.0.0" if not specified.
    #[serde(default = "default_host")]
    pub host: String,

    /// The port number to bind the server to.
    /// Defaults to 8000 if not specified.
    #[serde(default = "default_port")]
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

    /// Optional request timeout in seconds.
    /// If set, requests taking longer than this duration will be aborted.
    /// If not set, there is no timeout.
    pub request_timeout_seconds: Option<u64>,
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

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8000
}
