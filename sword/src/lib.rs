pub mod http {
    mod errors;
    mod request;

    pub use axum_responses::http::*;
    pub use axum_responses::{Result, response};
    pub use request::Request;
}

#[cfg(feature = "validation")]
mod validation;

pub mod routing {
    pub use sword_macros::{controller, delete, get, patch, post, put};
}

pub mod di {}
