use super::utils;
use serde::de::{DeserializeOwned, IntoDeserializer};
use std::{fs::read_to_string, path::Path, str::FromStr, sync::Arc};
use toml::Table;

pub use sword_macros::config;

use crate::errors::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    inner: Arc<Table>,
}

/// Trait for configuration section types.
///
/// Types implementing this trait can be used with `Config::get()` to extract
/// and deserialize specific sections from the configuration file.
///
/// Use the `#[config(key = "section_name")]` macro to automatically implement this trait:
///
/// ```rust,ignore
/// use sword::prelude::*;
///
/// #[config(key = "my_section")]
/// struct MyConfig {
///     value: String,
/// }
/// ```
pub trait ConfigItem {
    /// Returns the TOML section key for this configuration type.
    fn toml_key() -> &'static str;
}

impl Config {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let path = Path::new("config/config.toml");

        if !path.exists() {
            return Err(ConfigError::FileNotFound("config/toml"));
        }

        let content = read_to_string(path).map_err(ConfigError::ReadError)?;
        let expanded = utils::expand_env_vars(&content)
            .map_err(ConfigError::InterpolationError)?;

        let table = Table::from_str(&expanded)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(table),
        })
    }

    /// Retrieves and deserializes a configuration section.
    ///
    /// This method extracts a specific section from the loaded TOML configuration
    /// and deserializes it to the specified type. The type must implement both
    /// `DeserializeOwned` for parsing and `ConfigItem` to specify which section
    /// to load from.
    ///
    /// ### Type Parameters
    ///
    /// * `T` - The configuration type to deserialize (must implement `DeserializeOwned + ConfigItem`)
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// #[config(key = "application")]
    /// struct DatabaseConfig {
    ///     url: String,
    /// }
    ///
    /// // Then in a route handler:
    ///
    /// #[get("/db-info")]
    /// async fn db_info(ctx: Context) -> HttpResult<HttpResponse> {
    ///     let db_config = ctx.config::<DatabaseConfig>()?;
    ///     Ok(HttpResponse::Ok().data(db_config))
    /// }
    ///
    /// ```
    pub fn get<T: DeserializeOwned + ConfigItem>(&self) -> Result<T, ConfigError> {
        let config_item = match self.inner.get(T::toml_key()) {
            Some(value) => value,
            None => return Err(ConfigError::KeyNotFound(T::toml_key().to_string())),
        };

        let value = toml::Value::into_deserializer(config_item.clone());

        T::deserialize(value)
            .map_err(|e| ConfigError::DeserializeError(e.to_string()))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inner: Arc::new(Table::new()),
        }
    }
}
