use serde::{Deserialize, Serialize};

use sword::prelude::*;
use sword::web::HttpResult;

use serde_json::{json, Value};

#[derive(Deserialize, Debug, Serialize)]
#[config(key = "my-custom-section")]
pub struct MyConfig {
    custom_key: String,
}

#[controller("/")]
struct AppController {}

#[routes]
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
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let body = ctx.body::<Value>()?;
        let custom_config = ctx.config::<MyConfig>()?;

        Ok(HttpResponse::Ok()
            .data(json!({
                "received_data": body,
                "custom_config": custom_config
            }))
            .message("Data submitted successfully"))
    }

    #[get("/json")]
    async fn get_json() -> HttpResponse {
        HttpResponse::Ok().data(json!({ "foo": "bar" }))
    }
}

#[sword::main]
async fn main() {
    Application::builder()?
        .controller::<AppController>()
        .run()
        .await?;
}
