use std::{fs::read_to_string, path::Path, str::FromStr, sync::Arc};

use serde::de::{DeserializeOwned, IntoDeserializer};
use shellexpand::env as expand_env;

use toml::Table;

use crate::errors::ConfigError;

#[derive(Debug, Clone)]
pub struct SwordConfig {
    inner: Arc<Table>,
}

pub trait ConfigItem {
    fn toml_key() -> &'static str;
}

impl SwordConfig {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let path = Path::new("config/config.toml");

        if !path.exists() {
            return Err(ConfigError::FileNotFound("config/toml"));
        }

        let content = read_to_string(path).map_err(ConfigError::ReadError)?;

        let expanded = expand_env(&content)
            .map_err(|e| ConfigError::InterpolationError(e.to_string()))?
            .into_owned();

        let table =
            Table::from_str(&expanded).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(table),
        })
    }

    pub(crate) fn get<T: DeserializeOwned + ConfigItem>(&self) -> Result<T, ConfigError> {
        let config_item = match self.inner.get(T::toml_key()) {
            Some(value) => value,
            None => return Err(ConfigError::KeyNotFound(T::toml_key().to_string())),
        };

        let value = toml::Value::into_deserializer(config_item.clone());

        T::deserialize(value).map_err(|e| ConfigError::DeserializeError(e.to_string()))
    }
}

impl Default for SwordConfig {
    fn default() -> Self {
        Self {
            inner: Arc::new(Table::new()),
        }
    }
}
