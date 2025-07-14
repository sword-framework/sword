mod context;
mod middleware;
mod validation;

pub mod prelude {
    pub use crate::application::config::ConfigItem;
    pub use crate::application::{Application, ApplicationConfig, config_macro as config};

    pub use crate::errors::{ApplicationError, RequestError, StateError};

    pub use crate::web::*;
}

pub mod application;
pub mod errors;

pub mod web {
    pub use axum_responses::Result as HttpResult;
    pub use axum_responses::http::*;

    pub use crate::context::Context;
    pub use crate::middleware::*;
    pub use crate::next;

    pub use sword_macros::{controller, delete, get, patch, post, put, routes};

    use crate::application::SwordState;

    pub trait RouterProvider {
        fn router(state: SwordState) -> axum::routing::Router;
    }
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
