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
        match ctx.validated_query::<MyQuery>()? {
            Some(query) => Ok(HttpResponse::Ok()
                .data(query)
                .message("Hello with query parameters")),
            None => Ok(HttpResponse::Ok().message("Hello without query parameters")),
        }
    }

    #[post("/submit")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let body = ctx.validated_body::<MyBody>()?;

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()?
        .with_controller::<AppController>()
        .build();

    app.run().await?;
}
