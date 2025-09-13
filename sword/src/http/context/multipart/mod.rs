use axum::extract::FromRequest;
pub use axum::extract::multipart::*;

use crate::{errors::RequestError, web::Context};

impl Context {
    pub async fn multipart(&self) -> Result<Multipart, RequestError> {
        Ok(Multipart::from_request(self.clone().try_into()?, &()).await?)
    }
}

impl From<MultipartRejection> for RequestError {
    fn from(err: MultipartRejection) -> Self {
        RequestError::ParseError(
            "Failed to parse multipart form data",
            err.to_string(),
        )
    }
}

impl From<MultipartError> for RequestError {
    fn from(err: MultipartError) -> Self {
        RequestError::ParseError(
            "Failed to parse multipart form data",
            err.to_string(),
        )
    }
}
