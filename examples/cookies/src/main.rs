use sword::prelude::*;

struct SetCookieMw {}

impl Middleware for SetCookieMw {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        let cookies = ctx.cookies_mut().ok_or(HttpResponse::BadRequest())?;

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
        let cookies = ctx.cookies_mut().ok_or(HttpResponse::BadRequest())?;

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
        let cookies = ctx.cookies_mut().ok_or(HttpResponse::BadRequest())?;

        let session_cookie = cookies
            .get("session_id")
            .ok_or(HttpResponse::Unauthorized().message("Session cookie not found"))?;

        Ok(HttpResponse::Ok().message(format!("Session ID: {}", session_cookie.value())))
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()?.with_controller::<CookieController>().build();

    app.run().await?;
}
