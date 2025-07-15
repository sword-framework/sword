use serde::{Deserialize, Serialize};

use sword::prelude::*;
use sword::web::HttpResult;

mod http_logger;

use serde_json::json;

use crate::http_logger::HttpLogger;

#[derive(Deserialize, Debug, Serialize)]
#[config(key = "my-custom-section")]
pub struct MyConfig {
    custom_key: String,
}

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[post("/submit")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let custom_config = ctx.config::<MyConfig>()?;

        Ok(HttpResponse::Ok()
            .data(json!({
                "custom_config": custom_config
            }))
            .message("Data submitted successfully"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running multipart example");

    Application::builder()?
        .controller::<AppController>()
        .layer(HttpLogger::new().layer)
        .run()
        .await?;

    Ok(())
}
