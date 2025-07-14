use axum_responses::http::HttpResponse;
use serde_json::json;

use super::{RequestError, StateError};
use crate::errors::ConfigError;

impl From<RequestError> for HttpResponse {
    fn from(error: RequestError) -> HttpResponse {
        match error {
            RequestError::ParseError(message, details) => {
                HttpResponse::BadRequest().message(message).data(json!({
                    "type": "ParseError",
                    "details": details
                }))
            }
            RequestError::ValidationError(message, errors) => {
                HttpResponse::BadRequest().message(message).data(json!({
                    "type": "ValidationError",
                    "errors": errors
                }))
            }
            RequestError::BodyIsEmpty(message) => {
                HttpResponse::BadRequest().message(message).data(json!({
                    "type": "BodyEmpty",
                    "message": "Request body is empty or missing"
                }))
            }
            RequestError::BodyTooLarge => HttpResponse::PayloadTooLarge()
                .message("Request payload too large")
                .data(json!({
                    "type": "PayloadTooLarge",
                    "message": "The request body exceeds the maximum allowed size"
                })),
            RequestError::InvalidContentType(message) => HttpResponse::UnsupportedMediaType()
                .message("Invalid content type")
                .data(json!({
                    "type": "InvalidContentType",
                    "message": message
                })),
        }
    }
}

impl From<StateError> for HttpResponse {
    fn from(error: StateError) -> Self {
        match error {
            StateError::TypeNotFound => HttpResponse::InternalServerError()
                .message("Service configuration error")
                .data(json!({
                    "type": "ConfigurationError",
                    "message": "A required service is not available"
                })),
            StateError::LockError => HttpResponse::InternalServerError()
                .message("Internal server error")
                .data(json!({
                    "type": "InternalError",
                    "message": "A temporary server error occurred"
                })),
            StateError::DowncastFailed { .. } => HttpResponse::InternalServerError()
                .message("Internal server error")
                .data(json!({
                    "type": "InternalError",
                    "message": "An unexpected internal error occurred"
                })),
        }
    }
}

impl From<ConfigError> for HttpResponse {
    fn from(error: ConfigError) -> Self {
        match error {
            ConfigError::DeserializeError(message) => HttpResponse::InternalServerError()
                .message("Configuration error")
                .data(json!({
                    "type": "ConfigError",
                    "message": message
                })),
            ConfigError::KeyNotFound(key) => HttpResponse::InternalServerError()
                .message("Configuration error")
                .data(json!({
                    "type": "ConfigError",
                    "message": format!("Key '{}' not found in configuration", key)
                })),

            _ => HttpResponse::InternalServerError()
                .message("Configuration error")
                .data(json!({
                    "type": "ConfigError",
                    "message": "An error occurred while processing the app configuration"
                })),
        }
    }
}
