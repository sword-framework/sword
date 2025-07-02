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
    pub fn uri(&self) -> String {
        self.uri.to_string()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|value| value.as_str())
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    pub fn set_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(name.into(), value.into());
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
        let details = format!("Parameter '{key}' not found in request parameters");

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
