use std::{collections::HashMap, str::FromStr};

use axum::{
    body::{Body, Bytes, to_bytes},
    extract::{FromRef, FromRequest, OptionalFromRequestParts, Path, Request as AxumRequest},
    http::Method,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::{
    application::SwordState,
    http::{Context, HttpResponse, Result as HttpResult, errors::RequestError},
};

/// Implementation of `FromRequest` for `Context`.
///
/// Allows `Context` to be automatically extracted from HTTP requests
/// in Axum handlers, providing easy access to parameters, headers, body, and state.
impl<S> FromRequest<S> for Context
where
    S: Send + Sync + 'static,
    SwordState: FromRef<S>,
{
    type Rejection = HttpResponse;

    async fn from_request(req: AxumRequest, state: &S) -> HttpResult<Self> {
        let (mut parts, body) = req.into_parts();

        let mut params = HashMap::new();
        let path_result =
            Path::<HashMap<String, String>>::from_request_parts(&mut parts, &()).await;

        if let Ok(Some(path_params)) = path_result {
            params.extend(path_params.0);
        }

        let body_bytes = to_bytes(body, usize::MAX)
            .await
            .unwrap_or_else(|_| Bytes::new());

        let mut headers = HashMap::new();

        for (key, value) in parts.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        let state = SwordState::from_ref(state);

        Ok(Self {
            params,
            body_bytes,
            method: parts.method,
            headers,
            uri: parts.uri,
            extensions: parts.extensions,
            state,
        })
    }
}

impl Context {
    /// Gets the complete URI of the request as a string.
    ///
    /// # Returns
    /// A `String` containing the complete request URI including
    /// the path and query parameters if any.
    pub fn uri(&self) -> String {
        self.uri.to_string()
    }

    /// Gets the HTTP method of the request.
    ///
    /// # Returns
    /// A reference to the HTTP `Method` (GET, POST, PUT, DELETE, etc.).
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Gets the value of a specific header by name.
    ///
    /// # Arguments
    /// * `key` - The header name to search for (case-insensitive).
    ///
    /// # Returns
    /// `Some(&str)` with the header value if it exists, `None` if not found.
    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|value| value.as_str())
    }

    /// Gets an immutable reference to all request headers.
    ///
    /// # Returns
    /// A reference to `HashMap<String, String>` containing all request headers
    /// where the key is the header name and the value is its content.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Gets a mutable reference to all request headers.
    ///
    /// # Returns
    /// A mutable reference to `HashMap<String, String>` that allows modifying
    /// existing headers or adding new headers to the request.
    pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    /// Sets or updates the value of a header in the request.
    ///
    /// # Arguments
    /// * `name` - The header name to set. Must implement `Into<String>`.
    /// * `value` - The header value to set. Must implement `Into<String>`.
    ///
    /// # Note
    /// If the header already exists, its value will be overwritten.
    pub fn set_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(name.into(), value.into());
    }

    /// Gets and parses a route parameter by name.
    ///
    /// # Type Parameters
    /// * `T` - The type to convert the parameter to. Must implement `FromStr`.
    ///
    /// # Arguments
    /// * `key` - The name of the route parameter to get.
    ///
    /// # Returns
    /// `Ok(T)` with the parsed value if it exists and can be converted,
    /// `Err(RequestError)` if the parameter doesn't exist or cannot be parsed. This error
    /// can be automatically converted to an `HttpResponse` using the `?` operator.
    pub fn param<T: FromStr>(&self, key: &str) -> Result<T, RequestError> {
        if let Some(value) = self.params.get(key) {
            let Ok(param) = value.parse::<T>() else {
                let message = "Invalid parameter type";
                let details = format!(
                    "Failed to parse parameter '{}': expected type '{}', got '{}'",
                    key,
                    std::any::type_name::<T>(),
                    value
                );

                return Err(RequestError::ParseError(message, details));
            };

            return Ok(param);
        }

        let message = "Parameter not found";
        let details = format!("Parameter '{key}' not found in request parameters");

        Err(RequestError::ParseError(message, details))
    }

    /// Deserializes the request body to a specific type.
    ///
    /// # Type Parameters
    /// * `T` - The type to deserialize the JSON body to. Must implement `DeserializeOwned`.
    ///
    /// # Returns
    /// `Ok(T)` with the deserialized instance if the JSON is valid,
    /// `Err(RequestError)` if the body is empty or cannot be deserialized. This error
    /// can be automatically converted to an `HttpResponse` using the `?` operator.
    pub fn body<T: DeserializeOwned>(&self) -> Result<T, RequestError> {
        if self.body_bytes.is_empty() {
            return Err(RequestError::BodyIsEmpty(
                "Invalid call, request body is empty",
            ));
        }

        serde_json::from_slice(&self.body_bytes).map_err(|err| {
            let message = "Invalid request body";
            let details = format!(
                "Failed to parse request body to type '{}': {}",
                std::any::type_name::<T>(),
                err
            );

            RequestError::ParseError(message, details)
        })
    }

    /// Deserializes the query parameters (query string) to a specific type.
    ///
    /// # Type Parameters
    /// * `T` - The type to deserialize the query parameters to. Must implement `DeserializeOwned`.
    ///
    /// # Returns
    /// `Ok(T)` with the deserialized instance if the parameters are valid,
    /// `Err(RequestError)` if there's no query string or it cannot be deserialized. This error
    ///  can be automatically converted to an `HttpResponse` using the `?` operator.
    pub fn query<T: DeserializeOwned>(&self) -> Result<T, RequestError> {
        let query_str = self.uri.query();

        let Some(query_str) = query_str else {
            return Err(RequestError::ParseError(
                "Invalid query parameters",
                "Failed to parse - query string is empty".to_string(),
            ));
        };

        serde_qs::from_str(query_str).map_err(|err| {
            RequestError::ParseError(
                "Invalid request query",
                format!(
                    "Failed to parse request query to type '{}': {}",
                    std::any::type_name::<T>(),
                    err
                ),
            )
        })
    }

    /// Deserializes and validates the request body using validation rules.
    ///
    /// # Type Parameters
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned` and `Validate`.
    ///
    /// # Returns
    /// `Ok(T)` with the deserialized and validated instance,
    /// `Err(RequestError)` if there are deserialization or validation errors.
    ///
    /// # Errors
    /// - `RequestError::BodyIsEmpty` if the body is empty.
    /// - `RequestError::ParseError` if the JSON is invalid.
    /// - `RequestError::ValidationError` if the data doesn't pass validation rules.
    pub fn validated_body<T>(&self) -> Result<T, RequestError>
    where
        T: DeserializeOwned + Validate,
    {
        let body = self.body::<T>()?;

        body.validate().map_err(|error| {
            RequestError::ValidationError(
                "Invalid request body",
                crate::validation::format_validation_errors(&error),
            )
        })?;

        Ok(body)
    }

    /// Deserializes and validates the query parameters using validation rules.
    ///
    /// # Type Parameters
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned` and `Validate`.
    ///
    /// # Returns
    /// `Ok(T)` with the deserialized and validated instance,
    /// `Err(RequestError)` if there are deserialization or validation errors.
    ///
    /// # Errors
    /// - `RequestError::ParseError` if there's no query string or it cannot be parsed.
    /// - `RequestError::ValidationError` if the data doesn't pass validation rules.
    pub fn validated_query<T>(&self) -> Result<T, RequestError>
    where
        T: DeserializeOwned + Validate,
    {
        let query = self.query::<T>()?;

        query.validate().map_err(|error| {
            RequestError::ValidationError(
                "Invalid request query",
                crate::validation::format_validation_errors(&error),
            )
        })?;

        Ok(query)
    }
}

/// Implementation of conversion from `Context` to `AxumRequest`.
///
/// Allows converting a `Context` back to an Axum request,
/// preserving headers, method, URI, body, and extensions.
impl From<Context> for AxumRequest {
    fn from(req: Context) -> Self {
        use axum::http::{HeaderName, HeaderValue};

        let mut builder = AxumRequest::builder().method(req.method).uri(req.uri);

        for (key, value) in req.headers {
            if let (Ok(header_name), Ok(header_value)) =
                (key.parse::<HeaderName>(), value.parse::<HeaderValue>())
            {
                builder = builder.header(header_name, header_value);
            }
        }

        let body = Body::from(req.body_bytes);
        let mut request = builder.body(body).expect("Failed to build axum request");

        *request.extensions_mut() = req.extensions;

        request
    }
}
