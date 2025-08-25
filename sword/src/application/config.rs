use regex::Regex;
use serde::de::{DeserializeOwned, IntoDeserializer};
use std::{env, fs::read_to_string, path::Path, str::FromStr, sync::Arc};
use toml::Table;

use crate::errors::ConfigError;

#[derive(Debug, Clone)]
pub struct SwordConfig {
    inner: Arc<Table>,
}

pub trait ConfigItem {
    fn toml_key() -> &'static str;
}

fn expand_env_vars(content: &str) -> Result<String, String> {
    let re = Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*):?([^}]*)\}")
        .map_err(|e| format!("Regex error: {e}"))?;

    let mut result = content.to_string();

    for caps in re.captures_iter(content) {
        let full_match = caps.get(0).unwrap().as_str();
        let var_name = caps.get(1).unwrap().as_str();
        let default_value = caps.get(2).map(|m| m.as_str()).unwrap_or("");

        let replacement = match env::var(var_name) {
            Ok(value) => value,
            Err(_) => {
                if default_value.is_empty() {
                    return Err(format!("environment variable '{var_name}' not found"));
                } else {
                    default_value.to_string()
                }
            }
        };

        result = result.replace(full_match, &replacement);
    }

    let simple_re =
        Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").map_err(|e| format!("Regex error: {e}"))?;

    for caps in simple_re.captures_iter(&result.clone()) {
        let full_match = caps.get(0).unwrap().as_str();
        let var_name = caps.get(1).unwrap().as_str();

        if content.contains(&format!("${{{var_name}")) {
            continue;
        }

        let replacement = match env::var(var_name) {
            Ok(value) => value,
            Err(_) => return Err(format!("environment variable '{var_name}' not found")),
        };

        result = result.replace(full_match, &replacement);
    }

    Ok(result)
}

impl SwordConfig {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let path = Path::new("config/config.toml");

        if !path.exists() {
            return Err(ConfigError::FileNotFound("config/toml"));
        }

        let content = read_to_string(path).map_err(ConfigError::ReadError)?;
        let expanded = expand_env_vars(&content).map_err(ConfigError::InterpolationError)?;

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

impl Default for SwordConfig {
    fn default() -> Self {
        Self {
            inner: Arc::new(Table::new()),
        }
    }
}
