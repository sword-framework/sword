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

pub trait ConfigItem {
    fn toml_key() -> &'static str;
}

impl Config {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let path = Path::new("config/config.toml");

        if !path.exists() {
            return Err(ConfigError::FileNotFound("config/toml"));
        }

        let content = read_to_string(path).map_err(ConfigError::ReadError)?;
        let expanded = utils::expand_env_vars(&content).map_err(ConfigError::InterpolationError)?;

        let table =
            Table::from_str(&expanded).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(table),
        })
    }

    pub fn get<T: DeserializeOwned + ConfigItem>(&self) -> Result<T, ConfigError> {
        let config_item = match self.inner.get(T::toml_key()) {
            Some(value) => value,
            None => return Err(ConfigError::KeyNotFound(T::toml_key().to_string())),
        };

        let value = toml::Value::into_deserializer(config_item.clone());

        T::deserialize(value).map_err(|e| ConfigError::DeserializeError(e.to_string()))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inner: Arc::new(Table::new()),
        }
    }
}
