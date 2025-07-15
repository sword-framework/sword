use sword::prelude::*;
use sword::web::HttpResult;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[config(key = "my-custom-section")]
struct MyConfig {
    custom_key: String,
    env_var: String,
}

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/hello")]
    async fn hello(_: Context) -> HttpResult<HttpResponse> {
        Ok(HttpResponse::Ok().data("Hello, World from config example!"))
    }

    #[get("/config")]
    async fn show_config(ctx: Context) -> HttpResult<HttpResponse> {
        let config = ctx.config::<ApplicationConfig>()?;

        Ok(HttpResponse::Ok()
            .data(config)
            .message("This example demonstrates TOML config loading"))
    }

    #[get("/custom-conf")]
    async fn custom_config(ctx: Context) -> HttpResult<HttpResponse> {
        let custom_config = ctx.config::<MyConfig>()?;

        Ok(HttpResponse::Ok()
            .data(custom_config)
            .message("This example demonstrates custom config loading"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?.controller::<AppController>();

    app.run().await?;

    Ok(())
}
