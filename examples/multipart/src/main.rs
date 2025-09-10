use sword::prelude::*;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[post("/submit")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let form = ctx.multipart().await?;

        for field in form.fields() {
            println!("Field Name: {:?}", field.name);
        }

        Ok(HttpResponse::Ok().message("Data submitted successfully"))
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()?.with_controller::<AppController>().build();

    app.run().await?;
}
