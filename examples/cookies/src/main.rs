use sword::prelude::*;

#[derive(Clone)]
struct CookieKey(pub Key);

struct SetCookieMw {}

impl Middleware for SetCookieMw {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        let cookies = ctx.cookies_mut()?;

        let cookie = CookieBuilder::new("session_id", "abc123")
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .build();

        cookies.add(cookie);

        next!(ctx, next)
    }
}

#[controller("/cookies")]
struct CookieController {}

#[routes]
impl CookieController {
    #[get("/set")]
    async fn set_cookie(mut ctx: Context) -> HttpResult<HttpResponse> {
        let cookies = ctx.cookies_mut()?;

        let cookie = CookieBuilder::new("username", "sword_user")
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .build();

        cookies.add(cookie);

        Ok(HttpResponse::Ok())
    }

    #[get("/with_middleware")]
    #[middleware(SetCookieMw)]
    async fn with_middleware(mut ctx: Context) -> HttpResult<HttpResponse> {
        let cookies = ctx.cookies_mut()?;

        let session_cookie = cookies.get("session_id").ok_or(
            HttpResponse::Unauthorized().message("Session cookie not found"),
        )?;

        Ok(HttpResponse::Ok()
            .message(format!("Session ID: {}", session_cookie.value())))
    }

    #[get("/private-counter")]
    async fn private_counter(mut ctx: Context) -> HttpResult<HttpResponse> {
        let key = ctx.get_state::<CookieKey>()?;
        let private = ctx.cookies_mut()?.private(&key.0);

        let count = private
            .get("visited_private")
            .and_then(|c| c.value().parse::<u32>().ok())
            .unwrap_or(0);

        if count > 10 {
            private.remove(Cookie::new("visited_private", ""));

            Ok(HttpResponse::Ok()
                .message("You've visited more than 10 times, resetting counter."))
        } else {
            private.add(Cookie::new("visited_private", (count + 1).to_string()));

            Ok(HttpResponse::Ok()
                .message(format!("You've been {} times before", count)))
        }
    }
}

#[sword::main]
async fn main() {
    let my_key: &[u8] = &[0; 64];
    let key = Key::from(my_key);

    let app = Application::builder()?
        .with_state(CookieKey(key))?
        .with_controller::<CookieController>()
        .build();

    app.run().await?;
}
