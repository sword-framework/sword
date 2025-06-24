pub mod http;

#[cfg(feature = "validation")]
mod validation;

pub mod routing {
    use crate::application::AppState;

    use axum::routing::Router;
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
