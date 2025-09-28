use thiserror::Error;

mod mappers;

#[cfg(any(feature = "garde", feature = "validator"))]
mod formatting;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Failed to bind to address {address}: {source}")]
    BindFailed {
        address: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to start server: {source}")]
    ServerError {
        #[source]
        source: std::io::Error,
    },
    #[error("Config Error: {source}")]
    ConfigError {
        #[from]
        source: ConfigError,
    },
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error(
        "State type not found - ensure it is registered in the application state"
    )]
    TypeNotFound { type_name: String },

    #[error("Failed to acquire lock on state")]
    LockError,

    #[error("Failed to downcast state type '{type_name}'")]
    DowncastFailed { type_name: String },
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Failed to parse request: {0}")]
    ParseError(&'static str, String),

    #[cfg(feature = "validator")]
    #[error("Failed to validate request")]
    ValidatorError(&'static str, validator::ValidationErrors),

    #[cfg(feature = "garde")]
    #[error("Failed to validate request")]
    GardeError(&'static str, garde::Report),

    #[error("Request body is empty")]
    BodyIsEmpty(&'static str),

    #[error("Request body is too large")]
    BodyTooLarge,

    #[error("Unsupported media type: {0}")]
    UnsupportedMediaType(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration file not found at path: {0}")]
    FileNotFound(&'static str),

    #[error("Failed to read configuration file: {0}")]
    ReadError(std::io::Error),

    #[error("Failed to interpolate environment variables in configuration: {0}")]
    InterpolationError(String),

    #[error("Configuration key '{0}' not found")]
    KeyNotFound(String),

    #[error(
        "Configuration value for key '{key}' is invalid: {value}. Reason: {reason}"
    )]
    InvalidValue {
        key: String,
        value: String,
        reason: String,
    },
    #[error("Failed to build configuration: {0}")]
    BuildError(String),

    #[error("Failed to deserialize configuration: {0}")]
    DeserializeError(String),

    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    #[error("Error getting configuration from application state: {0}")]
    GetConfigError(String),
}
