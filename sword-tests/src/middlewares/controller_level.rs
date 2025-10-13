use axum_test::TestServer;
use serde_json::json;
use sword::prelude::*;

struct ExtensionsTestMiddleware;

impl Middleware for ExtensionsTestMiddleware {
    async fn handle(mut ctx: Context, nxt: Next) -> MiddlewareResult {
        ctx.extensions
            .insert::<String>("test_extension".to_string());

        next!(ctx, nxt)
    }
}

struct MwWithState;

impl Middleware for MwWithState {
    async fn handle(mut ctx: Context, nxt: Next) -> MiddlewareResult {
        ctx.extensions.insert::<u16>(8080);
        next!(ctx, nxt)
    }
}

struct RoleMiddleware;

impl MiddlewareWithConfig<Vec<&str>> for RoleMiddleware {
    async fn handle(
        roles: Vec<&str>,
        mut ctx: Context,
        next: Next,
    ) -> MiddlewareResult {
        ctx.extensions
            .insert::<String>(format!("Roles: {:?}", roles));

        next!(ctx, next)
    }
}

#[controller("/test")]
#[middleware(ExtensionsTestMiddleware)]
struct TestController {}

#[routes]
impl TestController {
    #[get("/extensions-test")]
    async fn extensions_test(&self, ctx: Context) -> HttpResponse {
        let extension_value = ctx.extensions.get::<String>();

        HttpResponse::Ok()
            .message("Test controller response with extensions")
            .data(json!({
                "extension_value": extension_value.cloned().unwrap_or_default()
            }))
    }

    #[get("/middleware-state")]
    #[middleware(MwWithState)]
    async fn middleware_state(&self, ctx: Context) -> HttpResult<HttpResponse> {
        let port = ctx.extensions.get::<u16>().cloned().unwrap_or(0);
        let message = ctx.extensions.get::<String>().cloned().unwrap_or_default();

        let json = json!({
            "port": port,
            "message": message
        });

        Ok(HttpResponse::Ok()
            .message("Test controller response with middleware state")
            .data(json))
    }

    #[get("/role-test")]
    #[middleware(RoleMiddleware, config = vec!["admin", "user"])]
    async fn role_test(&self) -> HttpResponse {
        HttpResponse::Ok()
    }
}

#[tokio::test]
async fn extensions_mw_test() {
    let app = Application::builder()
        .with_controller::<TestController>()
        .build();

    let test = TestServer::new(app.router()).unwrap();
    let response = test.get("/test/extensions-test").await;
    assert_eq!(response.status_code(), 200);

    let json = response.json::<ResponseBody>();

    assert!(json.data.is_some());

    let data = json.data.unwrap();

    assert_eq!(data["extension_value"], "test_extension");
}

#[tokio::test]
async fn middleware_state() {
    let app = Application::builder()
        .with_controller::<TestController>()
        .build();

    let test = TestServer::new(app.router()).unwrap();
    let response = test.get("/test/middleware-state").await;

    assert_eq!(response.status_code(), 200);

    let json = response.json::<ResponseBody>();

    assert!(json.data.is_some());

    let data = json.data.unwrap();

    assert_eq!(data["port"], 8080);
    assert_eq!(data["message"], "test_extension");
}

#[tokio::test]
async fn role_middleware_test() {
    let app = Application::builder()
        .with_controller::<TestController>()
        .build();

    let test = TestServer::new(app.router()).unwrap();
    let response = test.get("/test/role-test").await;

    assert_eq!(response.status_code(), 200);
}
