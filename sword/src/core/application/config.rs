use std::str::FromStr;

use byte_unit::Byte;
use serde::{Deserialize, Serialize};

use crate::core::ConfigItem;

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
/// request_timeout_seconds = 30
/// graceful_shutdown = true
/// ```
///
/// ### Environment Variable Interpolation
///
/// Configuration values support environment variable interpolation:
///
/// ```toml,ignore
/// [application]
/// host = "${HOST:127.0.0.1}"
/// port = "${PORT:3000}"
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
    pub body_limit: BodyLimit,

    /// Optional request timeout in seconds.
    /// If set, requests taking longer than this duration will
    /// be aborted and return a timeout error.
    ///
    /// If not set, there is no timeout.
    pub request_timeout_seconds: Option<u64>,

    /// Whether to enable graceful shutdown of the server.
    /// If true, the server will finish processing ongoing requests
    /// before shutting down when a termination signal is received.
    ///
    /// If you want to use a custom signal handler, you can disable this
    /// and implement your own signal with the `run_with_graceful_shutdown` method.
    #[serde(default = "default_graceful_shutdown")]
    pub graceful_shutdown: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BodyLimit {
    pub raw: String,
    pub parsed: usize,
}

impl<'de> Deserialize<'de> for BodyLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parsed = Byte::from_str(&s)
            .map(|b| b.as_u64() as usize)
            .map_err(serde::de::Error::custom)?;

        Ok(Self { raw: s, parsed })
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

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8000
}

fn default_graceful_shutdown() -> bool {
    false
}
