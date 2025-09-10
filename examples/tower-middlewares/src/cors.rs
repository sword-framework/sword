use std::collections::HashSet;
use std::str::FromStr;

use serde::Deserialize;
use sword::{
    core::config,
    web::{header, Method},
};

use tower_http::cors::CorsLayer;

#[derive(Clone, Debug, Deserialize)]
#[config(key = "cors")]
pub struct CorsConfig {
    pub allowed_http_methods: Vec<String>,
    pub allowed_http_headers: Vec<String>,
}

pub struct CorsMiddleware {
    pub layer: CorsLayer,
}

impl CorsMiddleware {
    pub fn new(config: CorsConfig) -> Self {
        let mut methods = HashSet::new();
        let mut headers = HashSet::new();

        for method in config.allowed_http_methods.iter() {
            let http_method =
                Method::from_str(method).expect("Invalid HTTP Method found in config");

            methods.insert(http_method);
        }

        for header in config.allowed_http_headers.iter() {
            let http_header =
                header::HeaderName::from_str(header).expect("Invalid HTTP Header found in config");

            headers.insert(http_header);
        }

        let methods = methods.into_iter().collect::<Vec<_>>();
        let headers = headers.into_iter().collect::<Vec<_>>();

        CorsMiddleware {
            layer: CorsLayer::new().allow_methods(methods).allow_headers(headers),
        }
    }
}
