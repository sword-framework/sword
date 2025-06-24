mod context;
mod errors;

use std::collections::HashMap;
use std::str::FromStr;

pub use axum_responses::http::*;
pub use axum_responses::{Result, response};
pub use context::Context;

use axum::http::Method;
use serde::de::DeserializeOwned;

use validator::Validate;

use crate::http::errors::RequestError;

pub trait RequestMethods {
    /// Returns the URI of the request as a string.
    fn uri(&self) -> String;

    /// Returns the HTTP method of the request.
    fn method(&self) -> &Method;

    /// Returns the value of a specific header by name.
    fn header(&self, name: &str) -> Option<&str>;

    /// Returns a reference to the headers of the request.
    fn headers(&self) -> &HashMap<String, String>;

    /// Returns a mutable reference to the headers of the request.
    fn headers_mut(&mut self) -> &mut HashMap<String, String>;

    /// Sets a header with the specified name and value.
    fn set_header(&mut self, name: impl Into<String>, value: impl Into<String>);

    /// Get a param and try parse to T.
    fn param<T: FromStr>(&self, key: &str) -> std::result::Result<T, RequestError>;

    /// Parses the body of the request into the specified type.
    fn body<T: DeserializeOwned>(&self) -> std::result::Result<T, RequestError>;

    /// Parses the query parameters of the request into the specified type.
    fn query<T: DeserializeOwned>(&self) -> std::result::Result<T, RequestError>;

    /// Validates and parses the body of the request into the specified type.
    fn validated_body<T: DeserializeOwned + Validate>(
        &self,
    ) -> std::result::Result<T, RequestError>;

    /// Validates and parses the query parameters of the request into the specified type.
    fn validated_query<T: DeserializeOwned + Validate>(
        &self,
    ) -> std::result::Result<T, RequestError>;
}
