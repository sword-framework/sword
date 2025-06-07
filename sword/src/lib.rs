mod request;

pub mod http {
    pub use super::request::Request;
    pub use axum_responses::http::*;
    pub use axum_responses::{Result, response};
}

pub mod routing {
    pub use sword_macros::{delete, get, patch, post, put, router};
}

pub mod di {}
