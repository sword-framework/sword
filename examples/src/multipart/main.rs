use sword::prelude::*;
use sword::web::HttpResult;

mod http_logger;
use crate::http_logger::HttpLogger;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[post("/submit")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let form = ctx.multipart().await?;

        for field in form {
            dbg!(&field);
        }

        Ok(HttpResponse::Ok().message("Data submitted successfully"))
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
