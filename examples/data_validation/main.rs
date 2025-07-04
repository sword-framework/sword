mod schemas;

use sword::http::Result as SwordResult;
use sword::prelude::*;

use schemas::{MyBody, MyQuery};

#[controller("/")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/hello")]
    async fn hello(ctx: Context) -> SwordResult<HttpResponse> {
        let query = ctx.validated_query::<MyQuery>()?;
        Ok(HttpResponse::Ok().data(query))
    }

    #[post("/submit")]
    async fn submit_data(ctx: Context) -> SwordResult<HttpResponse> {
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
