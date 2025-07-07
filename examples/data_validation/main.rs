mod schemas;

use sword::prelude::*;
use sword::web::HttpResult;

use schemas::{MyBody, MyQuery};

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/hello")]
    async fn hello(ctx: Context) -> HttpResult<HttpResponse> {
        let query = ctx.validated_query::<MyQuery>()?;
        Ok(HttpResponse::Ok().data(query))
    }

    #[post("/submit")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let body = ctx.validated_body::<MyBody>()?;

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    Application::builder()
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await?;

    Ok(())
}
