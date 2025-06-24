use serde_json::json;
use sword::{
    application::Application,
    controller::{controller, controller_impl},
    http::{Context, HttpResponse, ResponseBody, response},
    middleware::{Middleware, MiddlewareHandler, MiddlewareResult, NextFunction, middleware},
    routing::get,
};

#[derive(Middleware)]
struct ErrorMiddleware;

impl MiddlewareHandler for ErrorMiddleware {
    async fn handle(_: Context, _: NextFunction) -> MiddlewareResult {
        return Err(response!(500, { "message": "Internal Server Error" } ));
    }
}

#[derive(Middleware)]
struct ExtensionsTestMiddleware;

impl MiddlewareHandler for ExtensionsTestMiddleware {
    async fn handle(mut ctx: Context, next: NextFunction) -> MiddlewareResult {
        ctx.extensions
            .insert::<String>("test_extension".to_string());

        Ok(next.run(ctx).await)
    }
}

#[derive(Middleware)]
struct MiddlewareWithState;

impl MiddlewareHandler for MiddlewareWithState {
    async fn handle(mut ctx: Context, next: NextFunction) -> MiddlewareResult {
        ctx.extensions.insert::<u16>(8080);
        Ok(next.run(ctx).await)
    }
}

#[controller("/test")]
struct TestController {}

#[controller_impl]
impl TestController {
    #[get("/error-500")]
    #[middleware(ErrorMiddleware)]
    async fn hello() -> HttpResponse {
        HttpResponse::Ok()
            .data("Hello, World!")
            .message("Test controller response")
    }

    #[get("/extensions-test")]
    #[middleware(ExtensionsTestMiddleware)]
    async fn extensions_test(req: Context) -> HttpResponse {
        let extension_value = req.extensions.get::<String>();

        HttpResponse::Ok()
            .message("Test controller response with extensions")
            .data(json!({
                "extension_value": extension_value.cloned().unwrap_or_default()
            }))
    }

    #[get("/middleware-state")]
    #[middleware(MiddlewareWithState)]
    async fn middleware_state(req: Context) -> HttpResponse {
        let port = req.extensions.get::<u16>().cloned().unwrap_or(0);
        HttpResponse::Ok()
            .message("Test controller response with middleware state")
            .data(json!({ "port": port }))
    }
}

#[tokio::test]
async fn error_mw_test() {
    let app = Application::builder().controller::<TestController>();
    let test = axum_test::TestServer::new(app.router()).unwrap();
    let response = test.get("/test/error-500").await;
    assert_eq!(response.status_code(), 500);
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
async fn middleware_state_test() {
    let app = Application::builder().controller::<TestController>();
    let test = axum_test::TestServer::new(app.router()).unwrap();
    let response = test.get("/test/middleware-state").await;
    assert_eq!(response.status_code(), 200);
    let json = response.json::<ResponseBody>();

    let Some(data) = json.data else {
        panic!("Expected data in response");
    };

    assert_eq!(data["port"], 8080);
}
