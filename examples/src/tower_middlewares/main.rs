use sword::prelude::*;

use crate::{
    cors::{CorsConfig, CorsMiddleware},
    http_logger::HttpLogger,
};

mod cors;
mod http_logger;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/")]
    async fn get_data() -> HttpResponse {
        HttpResponse::Ok()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?;

    let cors_config = app.config.get::<CorsConfig>()?;
    let cors_middleware = CorsMiddleware::new(cors_config);

    let http_logger = HttpLogger::new();

    app.controller::<AppController>()
        .layer(http_logger.layer)
        .layer(cors_middleware.layer)
        .run()
        .await?;

    Ok(())
}
