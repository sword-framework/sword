use std::ops::Deref;

use axum_test::TestServer;
use serde_json::{Value, json};

use sword::prelude::*;
use sword::web::HttpResult;

#[controller("/test")]
struct TestController {}

#[routes]
impl TestController {
    #[get("/state")]
    async fn handler(ctx: Context) -> HttpResult<HttpResponse> {
        let data = ctx.get_state::<Value>()?;

        Ok(HttpResponse::Ok()
            .data(data.deref())
            .add_header("Content-Type", "application/json"))
    }
}

#[tokio::test]
async fn test_state() -> Result<(), Box<dyn std::error::Error>> {
    let data = json!({ "key": "value" });

    let app = Application::builder()?
        .state(data)?
        .controller::<TestController>();

    let server = TestServer::new(app.router()).unwrap();
    let response = server.get("/test/state").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );

    let json = response.json::<ResponseBody>();

    assert_eq!(json.data.get("key").unwrap(), "value");

    Ok(())
}
