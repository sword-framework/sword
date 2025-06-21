use sword::{
    application::Application,
    controller::{controller, controller_impl},
    http::{HttpResponse, Request},
    middleware::{Middleware, MiddlewareResult, NextFunction, middleware},
    routing::get,
};

#[derive(Middleware)]
struct SimpleMiddleware;

impl SimpleMiddleware {
    async fn handle(req: Request, next: NextFunction) -> MiddlewareResult {
        println!("First Mw: {}", req.uri());
        Ok(next.run(req).await)
    }
}

#[derive(Middleware)]
struct SimpleMiddleware2;

impl SimpleMiddleware2 {
    async fn handle(req: Request, next: NextFunction) -> MiddlewareResult {
        println!("Second Mw: {}", req.uri());
        Ok(next.run(req).await)
    }
}

#[controller("/test")]
struct TestController {}

#[controller_impl]
impl TestController {
    #[get("/hello")]
    #[middleware(SimpleMiddleware, SimpleMiddleware2)]
    async fn hello() -> HttpResponse {
        HttpResponse::Ok()
            .data("Hello, World!")
            .message("Test controller response")
    }
}

#[tokio::test]
async fn test_application() {
    let app = Application::new().add_controller::<TestController>();

    let test = axum_test::TestServer::new(app.router()).unwrap();

    let response = test.get("/test/hello").await;
    assert_eq!(response.status_code(), 200);
}
