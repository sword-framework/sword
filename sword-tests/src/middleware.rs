use serde_json::{Value, json};
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

        println!("2");

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

        println!("1");

        HttpResponse::Ok()
            .message("Test controller response with extensions")
            .data(json!({
                "extension_value": extension_value.cloned().unwrap_or_default()
            }))
    }

    #[get("/middleware-state")]
    #[middleware(ExtensionsTestMiddleware)]
    #[middleware(MwWithState)]
    async fn middleware_state(ctx: Context) -> Result<HttpResponse> {
        let port = ctx.extensions.get::<u16>().cloned().unwrap_or(0);
        let app_state = ctx.get_state::<Value>()?;
        let message = ctx.extensions.get::<String>().cloned().unwrap_or_default();

        let json = json!({
            "port": port,
            "key": app_state.get("key").and_then(Value::as_str).unwrap_or_default(),
            "message": message
        });

        Ok(HttpResponse::Ok()
            .message("Test controller response with middleware state")
            .data(json))
    }

    #[get("/role-test")]
    #[middleware(RoleMiddleware, config = vec!["admin", "user"])]
    async fn role_test(_: Context) -> HttpResponse {
        HttpResponse::Ok()
    }
}

#[tokio::test]
async fn extensions_mw_test() {
    let app = Application::builder().controller::<TestController>();
    let test = axum_test::TestServer::new(app.router()).unwrap();
    let response = test.get("/test/extensions-test").await;
    assert_eq!(response.status_code(), 200);

    let json = response.json::<ResponseBody>();

    let Some(data) = json.data else {
        panic!("Expected data in response");
    };

    assert_eq!(data["extension_value"], "test_extension");
}

#[tokio::test]
async fn middleware_state() {
    let app = Application::builder()
        .state(json!({ "key": "value" }))
        .controller::<TestController>();

    let test = axum_test::TestServer::new(app.router()).unwrap();
    let response = test.get("/test/middleware-state").await;

    assert_eq!(response.status_code(), 200);

    let json = response.json::<ResponseBody>();

    let Some(data) = json.data else {
        panic!("Expected data in response");
    };

    assert_eq!(data["port"], 8080);
    assert_eq!(data["key"], "value");
    assert_eq!(data["message"], "test_extension");
}

#[tokio::test]
async fn role_middleware_test() {
    let app = Application::builder().controller::<TestController>();
    let test = axum_test::TestServer::new(app.router()).unwrap();
    let response = test.get("/test/role-test").await;
    assert_eq!(response.status_code(), 200);
}
