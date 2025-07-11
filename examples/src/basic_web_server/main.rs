use sword::prelude::*;
use sword::web::HttpResult;

use serde_json::{json, Value};

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

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }

    #[get("/json")]
    async fn get_json() -> HttpResponse {
        HttpResponse::Ok().data(json!({ "foo": "bar" }))
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
