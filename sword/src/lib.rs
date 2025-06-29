pub mod prelude {
    pub use crate::application::Application;
    pub use crate::controller::{controller, controller_impl};
    pub use crate::extract::State;
    pub use crate::http::{HttpResponse, Request, RequestMethods, ResponseBody, response};
    pub use crate::middleware::*;
    pub use crate::routing::*;
}

pub mod di;

pub mod extract;
pub mod http;

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
    pub use axum::middleware::from_fn_with_state;
    pub use axum::response::IntoResponse;
    pub use axum::response::Response as AxumResponse;
    pub use axum::routing::{delete, get, patch, post, put};
}
