use crate::{errors::RequestError, web::Context};
use garde::Validate;
use serde::de::DeserializeOwned;

/// Trait for request validation using the `garde` crate.
///
/// This trait provides methods to validate and deserialize request data (body, query parameters, and path parameters)
/// using the `garde` validation framework. Unlike the `validator` crate, `garde` offers more advanced validation
/// features including custom validation contexts, conditional validation, and better error reporting.
///
/// # Features
///
/// - **Context-aware validation**: Support for custom validation contexts that can carry additional data
/// - **Conditional validation**: Rules that depend on other fields or external conditions
/// - **Custom validators**: User-defined validation functions
/// - **Better error messages**: More detailed and localized error reporting
///
/// # Comparison with Validator
///
/// While `validator` is simpler and has better performance for basic validations, `garde` provides:
/// - More flexible validation rules
/// - Better support for complex business logic validation
/// - Custom error messages per field
/// - Validation contexts for cross-field validation
///
/// # Feature Flag
///
/// This trait is only available when the `garde` feature is enabled in your `Cargo.toml`:
/// ```toml
/// sword = { version = "0.1.8", features = ["garde"] }
/// ```
///
/// # Examples
///
/// ## Basic Body Validation
/// ```rust,ignore
/// use garde::Validate;
/// use serde::Deserialize;
/// use sword::prelude::*;
///
/// #[derive(Deserialize, Validate)]
/// struct CreateUser {
///     #[garde(length(min = 2, max = 50))]
///     name: String,
///     #[garde(email)]
///     email: String,
///     #[garde(range(min = 18, max = 120))]
///     age: u32,
/// }
///
/// #[post("/users")]
/// async fn create_user(ctx: Context) -> HttpResult<HttpResponse> {
///     let user_data: CreateUser = ctx.body_garde()?;
///     // Data is guaranteed to be valid here
///     Ok(HttpResponse::Ok().data(user_data))
/// }
/// ```
///
/// ## Validation with Custom Context
/// ```rust,ignore
/// use garde::{Validate, ValidationContext};
/// use serde::Deserialize;
/// use sword::prelude::*;
///
/// #[derive(Deserialize, Validate)]
/// struct UpdatePassword {
///     current_password: String,
///     #[garde(custom(check_password_strength))]
///     new_password: String,
/// }
///
/// fn check_password_strength(
///     value: &str,
///     _ctx: &ValidationContext,
/// ) -> garde::Result {
///     if value.len() < 8 {
///         return Err(garde::Error::new("Password must be at least 8 characters"));
///     }
///     if !value.chars().any(|c| c.is_uppercase()) {
///         return Err(garde::Error::new("Password must contain uppercase letter"));
///     }
///     Ok(())
/// }
///
/// #[put("/password")]
/// async fn update_password(ctx: Context) -> HttpResult<HttpResponse> {
///     let password_data: UpdatePassword = ctx.body_garde()?;
///     Ok(HttpResponse::Ok().message("Password updated"))
/// }
/// ```
pub trait GardeRequestValidation {
    /// Deserializes and validates the request body using `garde` validation rules.
    ///
    /// This method combines JSON deserialization with `garde` validation. It first deserializes
    /// the request body into the target type, then runs all validation rules defined on that type.
    /// The validation context is created using the `Default` trait.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned + Validate`.
    ///         The `Validate::Context` associated type must implement `Default`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` with the deserialized and validated instance, or `Err(RequestError)`
    /// if deserialization or validation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The request body is empty (`RequestError::BodyIsEmpty`)
    /// - The JSON is malformed (`RequestError::ParseError`)
    /// - Validation rules fail (`RequestError::GardeError`)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use garde::Validate;
    /// use serde::Deserialize;
    /// use sword::prelude::*;
    ///
    /// #[derive(Deserialize, Validate)]
    /// struct LoginRequest {
    ///     #[garde(length(min = 3, max = 50))]
    ///     username: String,
    ///     #[garde(length(min = 8))]
    ///     password: String,
    /// }
    ///
    /// #[post("/login")]
    /// async fn login(ctx: Context) -> HttpResult<HttpResponse> {
    ///     let login_data: LoginRequest = ctx.body_garde()?;
    ///     // Process login...
    ///     Ok(HttpResponse::Ok().message("Login successful"))
    /// }
    /// ```
    fn body_garde<T: DeserializeOwned + Validate>(&self) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default;

    /// Deserializes and validates the request body using `garde` with a custom validation context.
    ///
    /// This method is similar to `body_garde()` but allows you to provide a custom validation context.
    /// This is useful when your validation rules need additional data that's not available in the
    /// struct itself, such as database connections, configuration values, or user permissions.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned + Validate`.
    ///
    /// # Parameters
    ///
    /// * `context` - The validation context to use. This will be passed to all validation rules.
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` with the deserialized and validated instance, or `Err(RequestError)`
    /// if deserialization or validation fails.
    ///
    /// # Errors
    ///
    /// Same as `body_garde()`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use garde::{Validate, ValidationContext};
    /// use serde::Deserialize;
    /// use sword::prelude::*;
    ///
    /// #[derive(Deserialize, Validate)]
    /// struct CreatePost {
    ///     #[garde(length(min = 1, max = 200))]
    ///     title: String,
    ///     #[garde(length(min = 10))]
    ///     content: String,
    ///     #[garde(custom(check_user_permissions))]
    ///     category: String,
    /// }
    ///
    /// struct PostContext {
    ///     user_id: u32,
    ///     is_admin: bool,
    /// }
    ///
    /// fn check_user_permissions(
    ///     category: &str,
    ///     ctx: &PostContext,
    /// ) -> garde::Result {
    ///     if category == "admin" && !ctx.is_admin {
    ///         return Err(garde::Error::new("Insufficient permissions"));
    ///     }
    ///     Ok(())
    /// }
    ///
    /// #[post("/posts")]
    /// async fn create_post(ctx: Context) -> HttpResult<HttpResponse> {
    ///     let post_ctx = PostContext { user_id: 1, is_admin: false };
    ///     let post_data: CreatePost = ctx.body_garde_with_context(post_ctx)?;
    ///     Ok(HttpResponse::Ok().data(post_data))
    /// }
    /// ```
    fn body_garde_with_context<T: DeserializeOwned + Validate>(
        &self,
        context: <T as Validate>::Context,
    ) -> Result<T, RequestError>;

    /// Deserializes and validates query parameters using `garde` validation rules.
    ///
    /// This method parses URL query parameters and validates them using `garde`. Since query
    /// parameters are optional in HTTP, this method returns `Option<T>` where `None` indicates
    /// no query parameters were present in the URL.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned + Validate`.
    ///         The `Validate::Context` associated type must implement `Default`.
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
    /// - Validation rules fail (`RequestError::GardeError`)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use garde::Validate;
    /// use serde::Deserialize;
    /// use sword::prelude::*;
    ///
    /// #[derive(Deserialize, Validate)]
    /// struct SearchQuery {
    ///     #[garde(length(min = 1, max = 100))]
    ///     q: Option<String>,
    ///     #[garde(range(min = 1, max = 1000))]
    ///     page: Option<u32>,
    ///     #[garde(range(min = 1, max = 100))]
    ///     limit: Option<u32>,
    /// }
    ///
    /// #[get("/search")]
    /// async fn search(ctx: Context) -> HttpResult<HttpResponse> {
    ///     let query: Option<SearchQuery> = ctx.query_garde()?;
    ///     let query = query.unwrap_or_default();
    ///     // Perform search...
    ///     Ok(HttpResponse::Ok().data(query))
    /// }
    /// ```
    fn query_garde<T: DeserializeOwned + Validate>(
        &self,
    ) -> Result<Option<T>, RequestError>
    where
        <T as Validate>::Context: Default;

    /// Deserializes and validates query parameters using `garde` with a custom validation context.
    ///
    /// This method is similar to `query_garde()` but allows you to provide a custom validation context.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned + Validate`.
    ///
    /// # Parameters
    ///
    /// * `context` - The validation context to use.
    ///
    /// # Returns
    ///
    /// Same as `query_garde()`.
    ///
    /// # Errors
    ///
    /// Same as `query_garde()`.
    fn query_garde_with_context<T: DeserializeOwned + Validate>(
        &self,
        context: <T as Validate>::Context,
    ) -> Result<Option<T>, RequestError>;

    /// Deserializes and validates path parameters using `garde` validation rules.
    ///
    /// This method extracts path parameters from the URL path and validates them using `garde`.
    /// Path parameters are typically extracted from route patterns like `/users/:id/posts/:post_id`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned + Validate`.
    ///         The `Validate::Context` associated type must implement `Default`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` with the deserialized and validated path parameters, or `Err(RequestError)`
    /// if deserialization or validation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Path parameters cannot be deserialized (`RequestError::ParseError`)
    /// - Validation rules fail (`RequestError::GardeError`)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use garde::Validate;
    /// use serde::Deserialize;
    /// use sword::prelude::*;
    ///
    /// #[derive(Deserialize, Validate)]
    /// struct PathParams {
    ///     #[garde(range(min = 1))]
    ///     user_id: u32,
    ///     #[garde(range(min = 1))]
    ///     post_id: u32,
    /// }
    ///
    /// #[get("/users/:user_id/posts/:post_id")]
    /// async fn get_post(ctx: Context) -> HttpResult<HttpResponse> {
    ///     let params: PathParams = ctx.params_garde()?;
    ///     // params.user_id and params.post_id are validated
    ///     Ok(HttpResponse::Ok().data(params))
    /// }
    /// ```
    fn params_garde<T: DeserializeOwned + Validate>(
        &self,
    ) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default;

    /// Deserializes and validates path parameters using `garde` with a custom validation context.
    ///
    /// This method is similar to `params_garde()` but allows you to provide a custom validation context.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize and validate. Must implement `DeserializeOwned + Validate`.
    ///
    /// # Parameters
    ///
    /// * `context` - The validation context to use.
    ///
    /// # Returns
    ///
    /// Same as `params_garde()`.
    ///
    /// # Errors
    ///
    /// Same as `params_garde()`.
    fn params_garde_with_context<T: DeserializeOwned + Validate>(
        &self,
        context: <T as Validate>::Context,
    ) -> Result<T, RequestError>;
}

#[cfg(feature = "garde")]
impl GardeRequestValidation for Context {
    fn body_garde<T: DeserializeOwned + Validate>(&self) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default,
    {
        let body = self.body::<T>()?;

        body.validate().map_err(|report| {
            RequestError::GardeError("Invalid request body", report)
        })?;

        Ok(body)
    }

    fn body_garde_with_context<T: DeserializeOwned + Validate>(
        &self,
        context: <T as Validate>::Context,
    ) -> Result<T, RequestError> {
        let body = self.body::<T>()?;

        body.validate_with(&context).map_err(|report| {
            RequestError::GardeError("Invalid request body", report)
        })?;

        Ok(body)
    }

    fn query_garde<T: DeserializeOwned + Validate>(
        &self,
    ) -> Result<Option<T>, RequestError>
    where
        <T as Validate>::Context: Default,
    {
        match self.query::<T>()? {
            Some(query) => {
                query.validate().map_err(|report| {
                    RequestError::GardeError("Invalid request query", report)
                })?;

                Ok(Some(query))
            }
            None => Ok(None),
        }
    }

    fn query_garde_with_context<T: DeserializeOwned + Validate>(
        &self,
        context: <T as Validate>::Context,
    ) -> Result<Option<T>, RequestError> {
        match self.query::<T>()? {
            Some(query) => {
                query.validate_with(&context).map_err(|report| {
                    RequestError::GardeError("Invalid request query", report)
                })?;

                Ok(Some(query))
            }
            None => Ok(None),
        }
    }

    fn params_garde<T: DeserializeOwned + Validate>(&self) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default,
    {
        let params = serde_json::to_value(self.params.clone()).map_err(|e| {
            RequestError::ParseError("Failed to serialize params", e.to_string())
        })?;

        let deserialized: T = serde_json::from_value(params).map_err(|e| {
            RequestError::ParseError(
                "Failed to deserialize params to the target type",
                e.to_string(),
            )
        })?;

        deserialized.validate().map_err(|report| {
            RequestError::GardeError("Invalid request params", report)
        })?;

        Ok(deserialized)
    }

    fn params_garde_with_context<T: DeserializeOwned + Validate>(
        &self,
        context: <T as Validate>::Context,
    ) -> Result<T, RequestError> {
        let params = serde_json::to_value(self.params.clone()).map_err(|e| {
            RequestError::ParseError("Failed to serialize params", e.to_string())
        })?;

        let deserialized: T = serde_json::from_value(params).map_err(|e| {
            RequestError::ParseError(
                "Failed to deserialize params to the target type",
                e.to_string(),
            )
        })?;

        deserialized.validate_with(&context).map_err(|report| {
            RequestError::GardeError("Invalid request params", report)
        })?;

        Ok(deserialized)
    }
}
