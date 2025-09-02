use serde_json::{json, Value};

use sword::prelude::*;
use sword::web::HttpResult;

mod middlewares;
use middlewares::*;

#[controller("/test")]
struct TestController {}

#[routes]
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
    async fn middleware_state(ctx: Context) -> HttpResult<HttpResponse> {
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
    #[middleware(MwWithState)]
    #[middleware(RoleMiddleware, config = vec!["admin", "user"])]
    async fn role_test(_: Context) -> HttpResponse {
        HttpResponse::Ok().message("Role middleware test passed")
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()?.with_controller::<TestController>().build();

    app.run().await?;
}
