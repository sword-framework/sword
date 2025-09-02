mod context;
mod middlewares;
mod validation;

pub mod prelude {
    pub use crate::core::{Application, ApplicationConfig};
    pub use crate::core::{ConfigItem, config};

    pub use crate::errors::{ApplicationError, RequestError, StateError};
    pub use crate::web::*;
}

pub mod errors;

pub mod core {
    mod application;
    mod config;
    mod router;
    mod state;
    mod utils;

    pub use router::RouterProvider;

    pub use application::{Application, ApplicationConfig};
    pub use config::{Config, ConfigItem, config};
    pub use state::State;
}

pub mod web {
    pub use axum_responses::Result as HttpResult;
    pub use axum_responses::http::*;
    pub use sword_macros::{controller, delete, get, patch, post, put, routes};

    pub use crate::context::Context;
    pub use crate::middlewares::*;
    pub use crate::next;

    #[cfg(feature = "multipart")]
    pub use crate::context::multipart::MultipartField;
}

pub use sword_macros::main;

#[cfg(feature = "hot-reload")]
pub mod hot_reload {
    pub use dioxus_devtools;
    pub use subsecond;
}

#[doc(hidden)]
pub mod __private {
    pub use axum::extract::{FromRequest, FromRequestParts, Request as AxumRequest};
    pub use axum::middleware::Next as AxumNext;
    pub use axum::middleware::from_fn_with_state as mw_with_state;
    pub use axum::response::{IntoResponse, Response as AxumResponse};
    pub use axum::routing::Router as AxumRouter;
    pub use axum::routing::{
        delete as axum_delete_fn, get as axum_get_fn, patch as axum_patch_fn, post as axum_post_fn,
        put as axum_put_fn,
    };
}
