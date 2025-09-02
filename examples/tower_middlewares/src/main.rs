mod cors;

use crate::cors::{CorsConfig, CorsMiddleware};
use sword::prelude::*;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/")]
    async fn get_data() -> HttpResponse {
        HttpResponse::Ok()
    }
}

#[sword::main]
async fn main() {
    let mut app = Application::builder()?;

    let cors_config = app.config.get::<CorsConfig>()?;
    let cors_middleware = CorsMiddleware::new(cors_config);

    app = app.with_controller::<AppController>().with_layer(cors_middleware.layer);

    app.build().run().await?;
}
