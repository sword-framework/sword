use axum_test::TestServer;
use serde_json::{Value, json};

use sword::core::Config;
use sword::prelude::*;
use sword::web::HttpResult;

#[controller("/test")]
struct TestController {}

#[routes]
impl TestController {
    #[get("/state")]
    async fn handler(&self, ctx: Context) -> HttpResult<HttpResponse> {
        let data = ctx.get_state::<Value>()?;

        Ok(HttpResponse::Ok()
            .data(data)
            .add_header("Content-Type", "application/json"))
    }
}

#[tokio::test]
async fn test_state() {
    let data = json!({ "key": "value" });

    let app = Application::builder()
        .with_state(data)
        .with_controller::<TestController>()
        .build();

    let server = TestServer::new(app.router()).unwrap();
    let response = server.get("/test/state").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );

    let json = response.json::<ResponseBody>();
    assert!(json.data.is_some());

    assert_eq!(json.data.unwrap().get("key").unwrap(), "value");
}

#[controller("/test-param-extraction")]
struct TestControllerParamExtraction {
    data: Value,
    config: Config,
}

#[routes]
impl TestControllerParamExtraction {
    #[get("/state")]
    async fn handler(&self) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok()
            .data(&self.data)
            .add_header("Content-Type", "application/json"))
    }

    #[get("/config/application")]
    async fn get_config(&self) -> HttpResult<HttpResponse> {
        let value = self.config.get::<ApplicationConfig>().unwrap_or_default();

        Ok(HttpResponse::Ok().data(&value))
    }
}

#[tokio::test]
async fn test_state_with_param_extraction() {
    let data = json!({ "key": "value" });

    let app = Application::builder()
        .with_state(data)
        .with_controller::<TestControllerParamExtraction>()
        .build();

    let server = TestServer::new(app.router()).unwrap();
    let response = server.get("/test-param-extraction/state").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );

    let json = response.json::<ResponseBody>();
    assert!(json.data.is_some());

    assert_eq!(json.data.unwrap().get("key").unwrap(), "value");
}

#[tokio::test]
async fn test_state_with_param_extraction_config() {
    let data = json!({ "key": "value" });

    let app = Application::builder()
        .with_state(data)
        .with_controller::<TestControllerParamExtraction>()
        .build();

    let server = TestServer::new(app.router()).unwrap();

    let response = server
        .get("/test-param-extraction/config/application")
        .await;

    assert_eq!(response.status_code(), 200);

    let json = response.json::<ResponseBody>();

    assert!(json.data.is_some());

    let data = serde_json::from_value::<ApplicationConfig>(json.data.unwrap())
        .inspect_err(|e| {
            panic!("Failed to deserialize ApplicationConfig: {}", e);
        });

    assert!(data.is_ok());

    // let config: ApplicationConfig =
    //     serde_json::from_value(json.data.unwrap()).unwrap();

    // assert_eq!(config.port, 8080);
    // assert_eq!(config.host, "0.0.0.0");
}
