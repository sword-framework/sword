use std::{collections::HashMap, str::FromStr};

use axum::{
    body::{Body, to_bytes},
    extract::{FromRef, FromRequest, Path, Request as AxumRequest},
    http::Method,
};

use http_body_util::LengthLimitError;
use serde::de::DeserializeOwned;

use validator::Validate;

use crate::{
    core::{Config, State},
    errors::RequestError,
    prelude::ApplicationConfig,
    web::{Context, HttpResponse, HttpResult},
};

/// Implementation of `FromRequest` for `Context`.
///
/// Allows `Context` to be automatically extracted from HTTP requests
/// in Axum handlers, providing easy access to parameters, headers, body, and state.
impl<S> FromRequest<S> for Context
where
    S: Send + Sync + 'static,
    State: FromRef<S>,
{
    type Rejection = HttpResponse;

    async fn from_request(req: AxumRequest, state: &S) -> HttpResult<Self> {
        let (mut parts, body) = req.into_parts();

        let mut params = HashMap::new();

        let path_result = {
            use axum::extract::OptionalFromRequestParts;
            Path::<HashMap<String, String>>::from_request_parts(&mut parts, &()).await
        };

        if let Ok(Some(path_params)) = path_result {
            params.extend(path_params.0);
        }

        let state = State::from_ref(state);

        let body_limit = state
            .get::<Config>()?
            .get::<ApplicationConfig>()
            .map(|app_config| app_config.body_limit)
            .unwrap_or(usize::MAX);

        let body_bytes = to_bytes(body, body_limit).await.map_err(|err| {
            let mut current_error: &dyn std::error::Error = &err;

            loop {
                if current_error.is::<LengthLimitError>() {
                    return RequestError::BodyTooLarge;
                }

                match std::error::Error::source(current_error) {
                    Some(source) => current_error = source,
                    None => break,
                }
            }

            RequestError::ParseError(
                "Failed to read request body",
                format!("Error reading body: {err}"),
            )
        })?;

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
        self.headers.get(&key.to_lowercase()).map(|value| value.as_str())
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

    /// Retrieves and parses a route parameter by name.
    /// 
    /// This method extracts URL parameters (path parameters) from the request
    /// and converts them to the specified type. The parameter must implement
    /// the `FromStr` trait for conversion.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The type to convert the parameter to (must implement `FromStr`)
    /// 
    /// # Arguments
    /// 
    /// * `key` - The name of the route parameter to extract
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(T)` with the parsed value if the parameter exists and can be
    /// converted, or `Err(RequestError)` if the parameter is missing or invalid.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The parameter is not found in the request
    /// - The parameter value cannot be parsed to type `T`
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// 
    /// // Route: GET /users/{id}/posts/{post_id}
    /// #[get("/users/{id}/posts/{post_id}")]
    /// async fn get_user_post(ctx: Context) -> HttpResult<String> {
    ///     let user_id: u32 = ctx.param("id")?;
    ///     let post_id: u64 = ctx.param("post_id")?;
    ///     
    ///     Ok(format!("User {} Post {}", user_id, post_id))
    /// }
    /// ```
    pub fn param<T: FromStr>(&self, key: &str) -> Result<T, RequestError> {
        if let Some(value) = self.params.get(key) {
            let Ok(param) = value.parse::<T>() else {
                let message = "Invalid parameter type";
                let details = "Failed to parse parameter to the required type";

                return Err(RequestError::ParseError(message, details.into()));
            };

            return Ok(param);
        }

        let message = "Parameter not found";
        let details = format!("Parameter '{key}' not found in request parameters");

        Err(RequestError::ParseError(message, details))
    }

    /// Deserializes the request body from JSON to a specific type.
    /// 
    /// This method reads the request body and attempts to parse it as JSON,
    /// deserializing it to the specified type. The body is consumed during
    /// this operation.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The type to deserialize the JSON body to (must implement `DeserializeOwned`)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(T)` with the deserialized instance if the JSON is valid,
    /// or `Err(RequestError)` if the body is empty or invalid JSON.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The request body is empty
    /// - The body contains invalid JSON
    /// - The JSON structure doesn't match the target type `T`
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use serde::Deserialize;
    /// 
    /// #[derive(Deserialize)]
    /// struct CreateUserRequest {
    ///     name: String,
    ///     email: String,
    ///     age: u32,
    /// }
    /// 
    /// #[post("/users")]
    /// async fn create_user(ctx: Context) -> HttpResult<String> {
    ///     let user_data: CreateUserRequest = ctx.body()?;
    ///     
    ///     // Process user creation...
    ///     
    ///     Ok(format!("Created user: {}", user_data.name))
    /// }
    /// ```
    pub fn body<T: DeserializeOwned>(&self) -> Result<T, RequestError> {
        if self.body_bytes.is_empty() {
            return Err(RequestError::BodyIsEmpty("Request body is empty"));
        }

        serde_json::from_slice(&self.body_bytes).map_err(|_| {
            let message = "Invalid request body";
            let details = "Failed to parse request body to the required type.";

            RequestError::ParseError(message, details.into())
        })
    }

    /// Deserializes query parameters from the URL query string to a specific type.
    /// 
    /// This method parses the query string portion of the URL and deserializes
    /// it to the specified type. Since query parameters are optional in HTTP,
    /// this method returns `Option<T>` where `None` indicates no query parameters
    /// were present.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The type to deserialize the query parameters to (must implement `DeserializeOwned`)
    /// 
    /// # Returns
    /// 
    /// Returns:
    /// - `Ok(Some(T))` with the deserialized query parameters if they exist and are valid
    /// - `Ok(None)` if no query parameters are present in the URL
    /// - `Err(RequestError)` if query parameters exist but cannot be deserialized
    /// 
    /// # Errors
    /// 
    /// This function will return an error if the query parameters exist but
    /// cannot be parsed or deserialized to the target type.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use serde::Deserialize;
    /// 
    /// #[derive(Deserialize, Default)]
    /// struct SearchQuery {
    ///     q: Option<String>,
    ///     page: Option<u32>,
    ///     limit: Option<u32>,
    /// }
    /// 
    /// // Route: GET /search?q=rust&page=1&limit=10
    /// #[get("/search")]
    /// async fn search(ctx: Context) -> HttpResult<String> {
    ///     let query: SearchQuery = ctx.query()?.unwrap_or_default();
    ///     
    ///     let search_term = query.q.unwrap_or_else(|| "all".to_string());
    ///     let page = query.page.unwrap_or(1);
    ///     let limit = query.limit.unwrap_or(20);
    ///     
    ///     Ok(format!("Searching '{}' - page {}, limit {}", search_term, page, limit))
    /// }
    /// ```
    pub fn query<T: DeserializeOwned>(&self) -> Result<Option<T>, RequestError> {
        let query_string = self.uri.query().unwrap_or("");

        if query_string.is_empty() {
            return Ok(None);
        }

        let deserializer =
            serde_urlencoded::Deserializer::new(form_urlencoded::parse(query_string.as_bytes()));

        let parsed: T = serde_path_to_error::deserialize(deserializer).map_err(|_| {
            let message = "Invalid query parameters";
            let details = "Failed to parse query parameters to the required type.";
            RequestError::ParseError(message, details.into())
        })?;

        Ok(Some(parsed))
    }

    /// Deserializes and validates the request body using validation rules.
    /// 
    /// This method combines JSON deserialization with validation using the
    /// `validator` crate. It first deserializes the request body and then
    /// runs validation rules defined on the target type.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The type to deserialize and validate (must implement `DeserializeOwned + Validate`)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(T)` with the deserialized and validated instance, or
    /// `Err(RequestError)` if there are deserialization or validation errors.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The request body is empty (`RequestError::BodyIsEmpty`)
    /// - The JSON is invalid (`RequestError::ParseError`)
    /// - The data fails validation rules (`RequestError::ValidationError`)
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use serde::Deserialize;
    /// use validator::Validate;
    /// 
    /// #[derive(Deserialize, Validate)]
    /// struct CreateUserRequest {
    ///     #[validate(length(min = 1, max = 50))]
    ///     name: String,
    ///     
    ///     #[validate(email)]
    ///     email: String,
    ///     
    ///     #[validate(range(min = 13, max = 120))]
    ///     age: u32,
    /// }
    /// 
    /// #[post("/users")]
    /// async fn create_user(ctx: Context) -> HttpResult<String> {
    ///     let user_data: CreateUserRequest = ctx.validated_body()?;
    ///     
    ///     // Data is guaranteed to be valid here
    ///     
    ///     Ok(format!("Created user: {}", user_data.name))
    /// }
    /// ```
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

    /// Deserializes and validates query parameters using validation rules.
    /// 
    /// This method combines query parameter parsing with validation using the
    /// `validator` crate. It first deserializes the query string and then
    /// runs validation rules defined on the target type.
    /// 
    /// Since query parameters are optional in HTTP, this method returns
    /// `Option<T>` where `None` indicates no query parameters were present.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The type to deserialize and validate (must implement `DeserializeOwned + Validate`)
    /// 
    /// # Returns
    /// 
    /// Returns:
    /// - `Ok(Some(T))` with the deserialized and validated query parameters if they exist and are valid
    /// - `Ok(None)` if no query parameters are present in the URL
    /// - `Err(RequestError)` if query parameters exist but fail deserialization or validation
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - Query parameters cannot be parsed (`RequestError::ParseError`)
    /// - The data fails validation rules (`RequestError::ValidationError`)
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// use sword::prelude::*;
    /// use serde::Deserialize;
    /// use validator::Validate;
    /// 
    /// #[derive(Deserialize, Validate, Default)]
    /// struct SearchQuery {
    ///     #[validate(length(min = 1, max = 100))]
    ///     q: Option<String>,
    ///     
    ///     #[validate(range(min = 1, max = 1000))]
    ///     page: Option<u32>,
    ///     
    ///     #[validate(range(min = 1, max = 100))]
    ///     limit: Option<u32>,
    /// }
    /// 
    /// // Route: GET /search?q=rust&page=1&limit=10
    /// #[get("/search")]
    /// async fn search(ctx: Context) -> HttpResult<String> {
    ///     let query: SearchQuery = ctx.validated_query()?.unwrap_or_default();
    ///     
    ///     // Query parameters are guaranteed to be valid here
    ///     
    ///     Ok(format!("Valid search query: {:?}", query))
    /// }
    /// ```
    pub fn validated_query<T>(&self) -> Result<Option<T>, RequestError>
    where
        T: DeserializeOwned + Validate,
    {
        match self.query::<T>()? {
            Some(query) => {
                query.validate().map_err(|error| {
                    RequestError::ValidationError(
                        "Invalid request query",
                        crate::validation::format_validation_errors(&error),
                    )
                })?;

                Ok(Some(query))
            }
            None => Ok(None),
        }
    }

    /// Checks if the request has a non-empty body.
    /// 
    /// This is an internal method used by the framework to determine
    /// if the request contains body data. It's primarily used for
    /// internal request processing logic.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the request has a body with content, `false` if empty.
    pub(crate) fn has_body(&self) -> bool {
        !self.body_bytes.is_empty()
    }
}

/// Implementation of conversion from `Context` to `AxumRequest`.
///
/// Allows converting a `Context` back to an Axum request,
/// preserving headers, method, URI, body, and extensions.
impl TryFrom<Context> for AxumRequest {
    type Error = RequestError;

    fn try_from(req: Context) -> Result<Self, Self::Error> {
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

        let mut request = builder.body(body).map_err(|_| {
            RequestError::ParseError(
                "Failed to build axum request",
                "Error building request".to_string(),
            )
        })?;

        *request.extensions_mut() = req.extensions;

        Ok(request)
    }
}
