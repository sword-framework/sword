use sword::prelude::*;
use sword::web::HttpResult;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[post("/submit")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let form = ctx.multipart().await?;

        for field in form.fields() {
            dbg!(&field);
        }

        Ok(HttpResponse::Ok().message("Data submitted successfully"))
    }
}

#[sword::main]
async fn main() {
    println!("Running multipart example");

    let app = Application::builder()?.with_controller::<AppController>().build();

    app.run().await?;
}
