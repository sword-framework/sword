use axum_responses::http::HttpResponse;
use serde_json::{Value, json};

#[derive(Debug)]
pub enum RequestError {
    ParseError(&'static str, String),
    #[cfg(feature = "validation")]
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
            #[cfg(feature = "validation")]
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
