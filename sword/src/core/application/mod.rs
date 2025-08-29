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

pub struct Application {
    router: Router,
    pub config: Config,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ApplicationConfig {
    pub host: String,
    pub port: u16,

    #[serde(deserialize_with = "deserialize_size")]
    pub body_limit: usize,

    #[cfg(feature = "multipart")]
    pub allowed_mime_types: Vec<String>,
}

impl Application {
    pub fn builder() -> Result<ApplicationBuilder, ApplicationError> {
        ApplicationBuilder::new()
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        let config = self.config.get::<ApplicationConfig>()?;
        let addr = format!("{}:{}", config.host, config.port);

        let listener = Listener::bind(&addr).await.map_err(|e| {
            ApplicationError::BindFailed {
                address: addr.to_string(),
                source: e,
            }
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

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub fn display(&self, config: &ApplicationConfig) {
        let ascii_logo = "\n▪──────── ⚔ S W O R D ⚔ ────────▪\n";
        println!("{ascii_logo}");
        println!("Starting Application ...");
        println!("Host: {}", config.host);
        println!("Port: {}", config.port);
        println!("{ascii_logo}");
    }
}

impl ConfigItem for ApplicationConfig {
    fn toml_key() -> &'static str {
        "application"
    }
}

fn deserialize_size<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Byte::from_str(&s)
        .map(|b| b.as_u64() as usize)
        .map_err(serde::de::Error::custom)
}
