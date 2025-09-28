use axum_test::TestServer;
use serde_json::{Value, json};

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
