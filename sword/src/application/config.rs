use std::path::Path;

use config::{Config as Cfg, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let toml_path = Path::new("config/config.toml");

        if !toml_path.exists() {
            return Err(config::ConfigError::Message(
                "Configuration file not found".to_string(),
            ));
        }

        let contents = std::fs::read_to_string(toml_path).map_err(|e| {
            config::ConfigError::Message(format!("Failed to read config file: {}", e))
        })?;

        let expanded = shellexpand::env(&contents)
            .map_err(|e| {
                config::ConfigError::Message(format!(
                    "Failed to expand environment variables: {}",
                    e
                ))
            })?
            .into_owned();

        Cfg::builder()
            .add_source(File::from_str(&expanded, config::FileFormat::Toml))
            .build()?
            .try_deserialize()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
        }
    }
}
