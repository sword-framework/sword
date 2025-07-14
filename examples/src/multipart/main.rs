use serde::{Deserialize, Serialize};

use sword::prelude::*;
use sword::web::HttpResult;

use serde_json::json;

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
    Application::builder()?
        .controller::<AppController>()
        .run()
        .await?;

    Ok(())
}
