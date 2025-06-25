pub mod prelude {
    pub use crate::application::{AppState, Application};
    pub use crate::controller::{controller, controller_impl};
    pub use crate::http::*;
    pub use crate::middleware::*;
    pub use crate::routing::*;
}

pub mod http;

mod validation;

pub mod routing {
    use crate::application::AppState;

    pub use axum::routing::Router;
    pub use sword_macros::{delete, get, patch, post, put};

    pub trait RouterProvider {
        fn router(app_state: AppState) -> Router;
    }
}

pub mod application;

pub mod controller {
    pub use sword_macros::{controller, controller_impl};
}

pub mod middleware;

pub(crate) mod utils {
    use std::fmt::Display;

    pub fn handle_critical_error<E: Display>(message: &str, error: E, lib: Option<&str>) -> ! {
        eprintln!("{}: {}", lib.unwrap_or("Sword"), message);
        eprintln!("Error: {}", error);
        std::process::exit(1)
    }
}

#[doc = "hidden"]
pub mod __private {
    pub use axum::extract::Request as AxumRequest;
    pub use axum::extract::State;
    pub use axum::extract::{FromRequest, FromRequestParts};
    pub use axum::middleware::Next as AxumNext;
    pub use axum::middleware::from_fn_with_state;
    pub use axum::response::IntoResponse;
    pub use axum::response::Response as AxumResponse;
    pub use axum::routing::{delete, get, patch, post, put};
}
