use std::{collections::HashMap, str::FromStr};

use axum::{
    body::{Bytes, to_bytes},
    extract::{FromRequest, OptionalFromRequestParts, Path, Request as AxumRequest},
    http::{Method, Uri},
};

#[cfg(feature = "validation")]
use validator::Validate;

use crate::http::{HttpResponse, Result as HttpResult, errors::RequestError};
use serde::de::DeserializeOwned;

pub struct Request {
    params: HashMap<String, String>,
    body_bytes: Bytes,
    method: Method,
    headers: HashMap<String, String>,
    uri: Uri,
}

impl<S> FromRequest<S> for Request
where
    S: Send + Sync + Clone,
{
    type Rejection = HttpResponse;

    async fn from_request(req: AxumRequest, _: &S) -> HttpResult<Self> {
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

        Ok(Self {
            params,
            body_bytes,
            method: parts.method,
            headers,
            uri: parts.uri,
        })
    }
}

impl Request {
    pub fn from_axum_request(req: AxumRequest) -> Self {
        let (parts, _body) = req.into_parts();

        let params = HashMap::new();
        let mut headers = HashMap::new();

        for (key, value) in parts.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        Self {
            params,
            body_bytes: Bytes::new(), // We'll need to handle this differently for middleware
            method: parts.method,
            headers,
            uri: parts.uri,
        }
    }

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
        let details = format!("Parameter '{}' not found in request parameters", key);

        Err(RequestError::ParseError(message, details))
    }

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

    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|value| value.as_str())
    }

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

    pub fn uri(&self) -> String {
        self.uri.to_string()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    #[cfg(feature = "validation")]
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

    #[cfg(feature = "validation")]
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

    pub fn into_axum_request(self) -> AxumRequest {
        use axum::http::{HeaderName, HeaderValue};

        let mut builder = axum::http::Request::builder()
            .method(self.method)
            .uri(self.uri);

        for (key, value) in self.headers {
            if let (Ok(header_name), Ok(header_value)) =
                (key.parse::<HeaderName>(), value.parse::<HeaderValue>())
            {
                builder = builder.header(header_name, header_value);
            }
        }

        let body = axum::body::Body::from(self.body_bytes);
        builder.body(body).expect("Failed to build axum request")
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            params: HashMap::new(),
            body_bytes: Bytes::new(),
            method: Method::GET,
            headers: HashMap::new(),
            uri: Uri::from_static("/"),
        }
    }
}

impl From<Request> for AxumRequest {
    fn from(req: Request) -> Self {
        req.into_axum_request()
    }
}
