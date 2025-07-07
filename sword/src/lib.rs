pub mod prelude {
    pub use crate::application::Application;
    pub use crate::controller::{controller, controller_impl};
    pub use crate::errors::{ApplicationError, RequestError, StateError};
    pub use crate::http::{Context, HttpResponse, ResponseBody};
    pub use crate::middleware::*;
    pub use crate::next;
    pub use crate::routing::*;
}

mod context;

pub mod http {
    pub use crate::context::Context;
    pub use axum_responses::http::*;
    pub use axum_responses::{Result, response};
}

pub mod errors;
mod validation;

pub mod routing {
    use crate::application::SwordState;

    pub use axum::routing::Router;
    pub use sword_macros::{delete, get, patch, post, put};

    pub trait RouterProvider {
        fn router(app_state: SwordState) -> Router;
    }
}

pub mod application;

pub mod controller {
    pub use sword_macros::{controller, controller_impl};
}

pub mod middleware;

#[doc = "hidden"]
pub mod __private {
    pub use axum::extract::Request as AxumRequest;
    pub use axum::extract::{FromRequest, FromRequestParts};
    pub use axum::middleware::Next as AxumNext;
    pub use axum::middleware::from_fn_with_state as mw_with_state;
    pub use axum::response::IntoResponse;
    pub use axum::response::Response as AxumResponse;
    pub use axum::routing::{
        delete as axum_delete_fn, get as axum_get_fn, patch as axum_patch_fn, post as axum_post_fn,
        put as axum_put_fn,
    };
}
