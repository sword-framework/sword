use axum_test::TestServer;
use serde_json::json;

use sword::prelude::*;

#[controller("/test")]
struct TestController {}

#[controller_impl]
impl TestController {
    #[get("/state")]
    async fn handler(ctx: Context) -> HttpResponse {
        let data = ctx
            .state
            .get::<serde_json::Value>()
            .expect("State not found");

        HttpResponse::Ok()
            .data(data)
            .add_header("Content-Type", "application/json")
    }
}

#[tokio::test]
async fn test_state() {
    let data = json!({ "key": "value" });

    let app = Application::builder()
        .state::<serde_json::Value>(data)
        .controller::<TestController>();

    let server = TestServer::new(app.router()).unwrap();
    let response = server.get("/test/state").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );

    let json = response.json::<ResponseBody>();

    let Some(data) = json.data else {
        panic!("Expected data in response");
    };

    assert_eq!(data.get("key").unwrap(), "value");
}
