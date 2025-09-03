use axum_responses::http::HttpResponse;
pub use tower_cookies::Cookies;
pub use tower_cookies::cookie::{Cookie, CookieBuilder, Expiration, SameSite};

use crate::web::Context;

impl Context {
    pub fn cookies(&self) -> Result<&Cookies, HttpResponse> {
        self.extensions.get::<Cookies>().ok_or(
            HttpResponse::InternalServerError()
                .message("Can't extract cookies. Is `CookieManagerLayer` enabled?"),
        )
    }

    pub fn cookies_mut(&mut self) -> Result<&mut Cookies, HttpResponse> {
        self.extensions.get_mut::<Cookies>().ok_or(
            HttpResponse::InternalServerError()
                .message("Can't extract cookies. Is `CookieManagerLayer` enabled?"),
        )
    }
}
