use serde_json::{json, Value};
use sword::http::Result;
use sword::prelude::*;

struct ExtensionsTestMiddleware;

impl Middleware for ExtensionsTestMiddleware {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        ctx.extensions
            .insert::<String>("test_extension".to_string());

        Ok(next.run(ctx.into()).await)
    }
}

struct MwWithState;

impl Middleware for MwWithState {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        let app_state = ctx.get_state::<Value>()?;

        ctx.extensions.insert::<u16>(8080);
        ctx.extensions.insert(app_state.clone());

        Ok(next.run(ctx.into()).await)
    }
}

struct RoleMiddleware;

impl MiddlewareWithConfig<Vec<&str>> for RoleMiddleware {
    async fn handle(roles: Vec<&str>, ctx: Context, next: Next) -> MiddlewareResult {
        dbg!(&roles);
        Ok(next.run(ctx.into()).await)
    }
}

#[controller("/test")]
struct TestController {}

#[controller_impl]
impl TestController {
    #[get("/extensions-test")]
    #[middleware(ExtensionsTestMiddleware)]
    async fn extensions_test(ctx: Context) -> HttpResponse {
        let extension_value = ctx.extensions.get::<String>();

        HttpResponse::Ok()
            .message("Test controller response with extensions")
            .data(json!({
                "extension_value": extension_value.cloned().unwrap_or_default()
            }))
    }

    #[get("/middleware-state")]
    #[middleware(MwWithState)]
    async fn middleware_state(ctx: Context) -> Result<HttpResponse> {
        let port = ctx.extensions.get::<u16>().cloned().unwrap_or(0);
        let app_state = ctx.get_state::<Value>()?;

        let json = json!({
            "port": port,
            "key": app_state.get("key").and_then(Value::as_str).unwrap_or_default()
        });

        Ok(HttpResponse::Ok()
            .message("Test controller response with middleware state")
            .data(json))
    }

    #[get("/role-test")]
    #[middleware(RoleMiddleware, config = vec!["admin", "user"])]
    async fn role_test(_: Context) -> HttpResponse {
        HttpResponse::Ok().message("Role middleware test passed")
    }
}

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<TestController>()
        .run("")
        .await;
}
