use std::process::Command;

use serde::{Deserialize, Serialize};
use sword::prelude::*;
use sword::web::HttpResult;

#[derive(Serialize, Deserialize, Debug)]
#[config(key = "my-custom-section")]
struct MyConfig {
    custom_key: String,
    env_user: String,
}

#[controller("/test")]
struct TestController {}

#[routes]
impl TestController {
    #[get("/hello")]
    async fn hello(ctx: Context) -> HttpResult<HttpResponse> {
        let custom_config = ctx.config::<MyConfig>()?;

        Ok(HttpResponse::Ok().data(custom_config).message("Test controller response"))
    }
}

#[tokio::test]
async fn test_application() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.with_controller::<TestController>().build();

    let test = axum_test::TestServer::new(app.router()).unwrap();

    let response = test.get("/test/hello").await;
    assert_eq!(response.status_code(), 200);

    let json_body = response.json::<ResponseBody>();

    let expected = MyConfig {
        custom_key: "value".to_string(),
        env_user: Command::new("sh")
            .arg("-c")
            .arg("echo $USER")
            .output()
            .expect("Failed to get environment variable")
            .stdout
            .into_iter()
            .map(|b| b as char)
            .collect::<String>()
            .trim()
            .to_string(),
    };

    assert_eq!(json_body.data["custom_key"], expected.custom_key);

    Ok(())
}
