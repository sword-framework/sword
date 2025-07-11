use axum::http::{HeaderName, Method};
use serde::Deserialize;
use thiserror::Error;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    application::config::{ConfigItem, SwordConfig},
    errors::ConfigError,
    layers::LayerProvider,
};

pub struct Cors {
    pub layer: CorsLayer,
    pub config: CorsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allowed_http_methods: Vec<String>,
    pub allowed_http_headers: Vec<String>,
    pub allow_credentials: bool,
}

impl Cors {
    pub fn new(config: &SwordConfig) -> Result<Self, ConfigError> {
        let mut layer = CorsLayer::new();

        let config = config.get::<CorsConfig>()?;

        if config.allowed_http_methods.iter().any(|m| m == "*") {
            layer = layer.allow_methods(Any);
        } else {
            let methods = Self::collect_http_methods(&config.allowed_http_methods)?;
            layer = layer.allow_methods(methods);
        }

        if config.allowed_http_headers.iter().any(|h| h == "*") {
            layer = layer.allow_headers(Any);
        } else {
            let headers = Self::collect_http_headers(&config.allowed_http_headers)?;
            layer = layer.allow_headers(headers);
        }

        if config.allow_credentials {
            layer = layer.allow_credentials(true);
        }

        Ok(Self { layer, config })
    }

    pub fn collect_http_methods(input: &[String]) -> Result<Vec<Method>, ConfigError> {
        input
            .iter()
            .map(|method_str| {
                method_str.parse::<Method>().map_err(|_| {
                    ConfigError::CorsError(CorsError::InvalidMethod {
                        key: "allowed_http_methods".to_string(),
                        value: method_str.clone(),
                    })
                })
            })
            .collect()
    }

    pub fn collect_http_headers(input: &[String]) -> Result<Vec<HeaderName>, ConfigError> {
        input
            .iter()
            .map(|header_str| {
                header_str.parse::<HeaderName>().map_err(|_| {
                    ConfigError::CorsError(CorsError::InvalidHeader {
                        key: "allowed_http_headers".to_string(),
                        value: header_str.clone(),
                    })
                })
            })
            .collect()
    }
}

impl LayerProvider<CorsLayer, CorsConfig> for Cors {
    fn layer(&self) -> CorsLayer {
        self.layer.clone()
    }

    fn config(&self) -> &CorsConfig {
        &self.config
    }
}

impl ConfigItem for CorsConfig {
    fn toml_key() -> &'static str {
        "cors"
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_http_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "PATCH".to_string(),
                "DELETE".to_string(),
            ],
            allowed_http_headers: vec!["*".to_string()],
            allow_credentials: false,
            enabled: true,
        }
    }
}

#[derive(Debug, Error)]
pub enum CorsError {
    #[error("Invalid HTTP method `{value}` in `{key}`")]
    InvalidMethod { key: String, value: String },
    #[error("Invalid header `{value}` in `{key}`")]
    InvalidHeader { key: String, value: String },
}
