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
    pub use sword_macros::{delete, get, patch, post, put};

    pub trait RouterProvider {
        fn router() -> axum::Router;
    }
}

pub mod di {}

pub mod application;

pub mod controller {
    pub use sword_macros::{Controller, controller_impl};

    pub trait ControllerKind {
        fn name() -> &'static str;
        fn file_path() -> &'static str;
    }
}

pub(crate) mod utils {
    use std::fmt::Display;

    pub fn handle_critical_error<E: Display>(message: &str, error: E, lib: Option<&str>) -> ! {
        eprintln!("{}: {}", lib.unwrap_or("Sword"), message);
        eprintln!("Error: {}", error);
        std::process::exit(1)
    }
}
