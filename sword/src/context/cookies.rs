pub use tower_cookies::Cookies;
pub use tower_cookies::cookie::{Cookie, CookieBuilder, Expiration, SameSite};

use crate::web::Context;

impl Context {
    pub fn cookies(&self) -> Option<&Cookies> {
        self.extensions.get::<Cookies>()
    }

    pub fn cookies_mut(&mut self) -> Option<&mut Cookies> {
        self.extensions.get_mut::<Cookies>()
    }
}
