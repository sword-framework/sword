use std::time::Duration;
use sword::prelude::*;
use tokio::time::sleep;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/")]
    async fn get_data(&self, _: Context) -> HttpResponse {
        sleep(Duration::from_secs(5)).await;
        HttpResponse::Ok()
    }
}

#[controller("/admin")]
struct AdminController {}

#[routes]
impl AdminController {
    #[get("/")]
    async fn get_admin_data(&self, _: Context) -> HttpResponse {
        HttpResponse::Ok()
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()
        .with_controller::<AppController>()
        .with_controller::<AdminController>()
        .build();

    app.run().await;
}
