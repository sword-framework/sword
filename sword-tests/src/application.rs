use sword::prelude::*;

#[controller("/test")]
struct TestController {}

#[routes]
impl TestController {
    #[get("/hello")]
    async fn hello() -> HttpResponse {
        HttpResponse::Ok()
            .data("Hello, World!")
            .message("Test controller response")
    }
}

#[controller("/second")]
struct SecondTestController {}

#[routes]
impl SecondTestController {
    #[get("/greet")]
    async fn greet() -> HttpResponse {
        HttpResponse::Ok()
            .data("Greetings from SecondTestController!")
            .message("Second test controller response")
    }
}

#[tokio::test]
async fn test_application() {
    let app = Application::builder()
        .controller::<TestController>()
        .controller::<SecondTestController>();

    let test = axum_test::TestServer::new(app.router()).unwrap();

    let response = test.get("/test/hello").await;
    assert_eq!(response.status_code(), 200);

    let second_response = test.get("/second/greet").await;
    assert_eq!(second_response.status_code(), 200);
}
