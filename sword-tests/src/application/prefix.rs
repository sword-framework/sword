use axum_test::TestServer;
use std::sync::LazyLock;
use sword::prelude::*;

static APPLICATION: LazyLock<TestServer> = LazyLock::new(|| {
    let app = Application::builder()
        .with_controller::<RootController>()
        .with_prefix("/api")
        .build();

    TestServer::new(app.router()).unwrap()
});

#[controller("/", version = "v1")]
pub struct RootController;

#[routes]
impl RootController {
    #[get("/")]
    async fn index(&self) -> HttpResponse {
        HttpResponse::Ok().message("Hello, World!")
    }
}

#[tokio::test]
async fn test_global_prefix() {
    let response = APPLICATION.get("/api/v1/").await;
    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(body.message, "Hello, World!".into());
}
