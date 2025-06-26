use serde_json::{Value, json};
use sword::prelude::*;

struct ErrorMiddleware;

impl Middleware for ErrorMiddleware {
    async fn handle(_: Request, _: Next) -> MiddlewareResult {
        return Err(response!(500, { "message": "Internal Server Error" } ));
    }
}

struct ExtensionsTestMiddleware;

impl Middleware for ExtensionsTestMiddleware {
    async fn handle(mut req: Request, next: Next) -> MiddlewareResult {
        req.extensions
            .insert::<String>("test_extension".to_string());

        Ok(next.run(req.into()).await)
    }
}

struct MwWithState;

impl MiddlewareWithState<Value> for MwWithState {
    async fn handle(state: State<Value>, mut req: Request, next: Next) -> MiddlewareResult {
        req.extensions.insert::<u16>(8080);
        req.extensions.insert(state.clone());

        Ok(next.run(req.into()).await)
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
    async fn extensions_test(req: Request) -> HttpResponse {
        let extension_value = req.extensions.get::<String>();

        HttpResponse::Ok()
            .message("Test controller response with extensions")
            .data(json!({
                "extension_value": extension_value.cloned().unwrap_or_default()
            }))
    }

    #[get("/middleware-state")]
    #[middleware(MwWithState)]
    async fn middleware_state(State(state): State<Value>, req: Request) -> Result<HttpResponse> {
        let port = req.extensions.get::<u16>().cloned().unwrap_or(0);

        Ok(HttpResponse::Ok()
            .message("Test controller response with middleware state")
            .data(json!({
                "port": port,
                "key": state.get("key").and_then(Value::as_str).unwrap_or_default()
            })))
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
}
