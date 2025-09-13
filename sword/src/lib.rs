mod validation;

/// The prelude module contains the most commonly used items from the Sword framework.
///
/// This module is designed to be imported with a glob import to bring all essential
/// types and traits into scope for typical Sword applications.
///
/// ### Example
///
/// ```rust,ignore
/// use sword::prelude::*;
///
/// // Now you have access to Application, Context, HttpResult, and more
/// ```
pub mod prelude {
    pub use crate::core::{Application, ApplicationConfig};
    pub use crate::core::{ConfigItem, config};

    pub use crate::errors::{ApplicationError, RequestError, StateError};
    pub use crate::web::*;

    #[cfg(feature = "cookies")]
    pub use crate::web::cookies::*;

    #[cfg(feature = "multipart")]
    pub use crate::web::multipart;
}

pub mod errors;

pub mod core {
    mod application;
    mod config;
    mod router;
    mod state;
    mod utils;

    pub use router::RouterProvider;
    pub use utils::deserialize_size;

    pub use application::{Application, ApplicationConfig};
    pub use config::{Config, ConfigItem, config};
    pub use state::State;
}

pub mod web {

    mod context;
    mod middleware;

    pub use axum::http::{Method, StatusCode, header};
    pub use axum_responses::Result as HttpResult;
    pub use axum_responses::http::*;
    pub use sword_macros::{controller, delete, get, patch, post, put, routes};

    pub use crate::next;
    pub use context::{Context, request::RequestValidation};
    pub use middleware::*;

    #[cfg(feature = "multipart")]
    pub use context::multipart;

    #[cfg(feature = "cookies")]
    pub use context::cookies;
}

pub use sword_macros::main;

#[doc(hidden)]
pub mod __internal {
    pub use axum::extract::{FromRequest, FromRequestParts, Request as AxumRequest};
    pub use axum::middleware::Next as AxumNext;
    pub use axum::middleware::from_fn_with_state as mw_with_state;
    pub use axum::response::{IntoResponse, Response as AxumResponse};
    pub use axum::routing::Router as AxumRouter;
    pub use axum::routing::{
        delete as axum_delete_fn, get as axum_get_fn, patch as axum_patch_fn,
        post as axum_post_fn, put as axum_put_fn,
    };

    #[cfg(feature = "hot-reload")]
    pub mod hot_reload {
        pub use dioxus_devtools;
        pub use subsecond;
    }

    pub use tokio::runtime as tokio_runtime;
}
