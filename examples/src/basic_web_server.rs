use sword::{
    application::Application,
    controller::{controller, controller_impl},
    http::{Context, HttpResponse, RequestMethods, Result},
    routing::{get, post},
};

#[controller("/")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/")]
    async fn get_data() -> HttpResponse {
        let data = vec![
            "This is a basic web server",
            "It serves static data",
            "You can extend it with more routes",
        ];

        HttpResponse::Ok().data(data)
    }

    #[get("/hello")]
    async fn hello() -> HttpResponse {
        HttpResponse::Ok().data("Hello, World!")
    }

    #[post("/submit")]
    async fn submit_data(ctx: Context) -> Result<HttpResponse> {
        let body = ctx.body::<serde_json::Value>()?;

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }
}

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<AppController>()
        .run()
        .await;
}
