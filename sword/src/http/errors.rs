use axum_responses::http::HttpResponse;
use serde_json::{Value, json};

#[derive(Debug)]
pub enum RequestError {
    ParseError(&'static str, String),
    ValidationError(&'static str, Value),
    BodyIsEmpty(&'static str),
}

impl From<RequestError> for HttpResponse {
    fn from(error: RequestError) -> Self {
        match error {
            RequestError::ParseError(message, details) => {
                HttpResponse::BadRequest().message(message).data(json!({
                    "type": "ParseError",
                    "message": details
                }))
            }
            RequestError::ValidationError(message, details) => {
                HttpResponse::BadRequest().message(message).data(json!({
                    "type": "ValidationError",
                    "errors": details
                }))
            }
            RequestError::BodyIsEmpty(message) => {
                HttpResponse::BadRequest().message(message).data(json!({
                    "type": "BodyError",
                    "message": "Request body is empty"
                }))
            }
        }
    }
}

pub enum ContextError {
    StateNotFound(&'static str),
    DependencyNotFound(&'static str),
}

impl From<ContextError> for HttpResponse {
    fn from(error: ContextError) -> Self {
        match error {
            ContextError::StateNotFound(type_name) => HttpResponse::InternalServerError()
                .message("State not found")
                .data(json!({
                    "type": "StateNotFound",
                    "message": format!("State of type '{}' not found", type_name)
                })),

            ContextError::DependencyNotFound(type_name) => HttpResponse::InternalServerError()
                .message("Dependency not found")
                .data(json!({
                    "type": "DependencyNotFound",
                    "message": format!("Dependency of type '{}' not found", type_name)
                })),
        }
    }
}
