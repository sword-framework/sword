use axum_responses::http::HttpResponse;
use serde_json::json;

use super::{RequestError, StateError};
use crate::errors::ConfigError;

#[cfg(feature = "validator")]
use crate::errors::formatting::format_validator_errors;

#[cfg(feature = "garde")]
use crate::errors::formatting::format_garde_errors;

impl From<RequestError> for HttpResponse {
    fn from(error: RequestError) -> HttpResponse {
        match error {
            RequestError::ParseError(message, details) => {
                HttpResponse::BadRequest().message(message).data(json!({
                    "type": "ParseError",
                    "details": details
                }))
            }

            #[cfg(feature = "validator")]
            RequestError::ValidatorError(message, errors) => {
                HttpResponse::BadRequest()
                    .message(message)
                    .data(format_validator_errors(errors))
            }

            #[cfg(feature = "garde")]
            RequestError::GardeError(message, errors) => HttpResponse::BadRequest()
                .message(message)
                .data(format_garde_errors(errors)),

            RequestError::BodyIsEmpty(message) => {
                HttpResponse::BadRequest().message(message)
            }
            RequestError::BodyTooLarge => HttpResponse::PayloadTooLarge().message(
                "The request body exceeds the maximum allowed size by the server",
            ),

            RequestError::UnsupportedMediaType(message) => {
                HttpResponse::UnsupportedMediaType().message(message)
            }

            RequestError::InternalError(message) => {
                eprintln!("Internal server error: {message}");
                HttpResponse::InternalServerError().message("Internal server error")
            }
        }
    }
}

impl From<StateError> for HttpResponse {
    fn from(error: StateError) -> Self {
        match error {
            StateError::TypeNotFound { type_name } => {
                eprintln!("Type: {type_name} not found on state");
                HttpResponse::InternalServerError()
                    .message("Service configuration error")
            }
            StateError::LockError => HttpResponse::InternalServerError(),
            StateError::DowncastFailed { type_name } => {
                eprintln!("Error downcasting type {type_name} from state");
                HttpResponse::InternalServerError().message("Internal server error")
            }
        }
    }
}

impl From<ConfigError> for HttpResponse {
    fn from(error: ConfigError) -> Self {
        match error {
            ConfigError::DeserializeError(message) => {
                eprintln!("Configuration error: {message}");
                HttpResponse::InternalServerError().message("Configuration error")
            }
            ConfigError::KeyNotFound(key) => {
                let message = format!("Key '{key}' not found in configuration");
                eprintln!("{message}");
                HttpResponse::InternalServerError().message("Configuration error")
            }

            _ => HttpResponse::InternalServerError()
                .message("Configuration error")
                .message("An error occurred while processing the app configuration"),
        }
    }
}
