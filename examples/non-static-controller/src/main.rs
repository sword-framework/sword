use sword::prelude::*;

#[controller("/")]
struct AppController {
    key: String,
}

#[routes]
impl AppController {
    #[get("/")]
    async fn get_data(&self, _: Context) -> HttpResponse {
        HttpResponse::Ok().data(&self.key)
    }

    #[get("/info")]
    async fn get_info(&self, _: Context) -> HttpResponse {
        HttpResponse::Ok().data(format!("Info: {}", &self.key))
    }

    #[post("/update")]
    async fn update_data(&self, _: Context) -> HttpResponse {
        HttpResponse::Ok().message("Data updated with key: ".to_string() + &self.key)
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()?
        .with_state(String::from("Hello, Sword!"))?
        .with_controller::<AppController>()
        .build();

    app.run().await?;
}
