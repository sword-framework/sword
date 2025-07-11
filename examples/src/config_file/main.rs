use sword::prelude::*;
use sword::web::HttpResult;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/hello")]
    async fn hello(_ctx: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok().data("Hello, World from config example!"))
    }

    #[get("/config")]
    async fn show_config(_: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok()
            .data("Configuration loaded successfully")
            .message("This example demonstrates TOML config loading"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.controller::<AppController>();

    app.run().await?;

    Ok(())
}
