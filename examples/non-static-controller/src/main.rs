use serde::{Deserialize, Serialize};
use sword::prelude::*;

mod database;
use crate::database::InMemoryDatabase;

#[controller("/")]
struct AppController {
    db: InMemoryDatabase,
    config: CustomConfig,
}

#[derive(Serialize, Deserialize)]
#[config(key = "custom-config")]
struct CustomConfig {
    some_variable: String,
}

#[routes]
impl AppController {
    #[get("/")]
    async fn get_names(&self, _: Context) -> HttpResponse {
        HttpResponse::Ok().data(self.db.read())
    }

    #[post("/{name}")]
    async fn add_name(&self, ctx: Context) -> HttpResult<HttpResponse> {
        self.db.write(ctx.param::<String>("name")?);

        Ok(HttpResponse::Created())
    }

    #[get("/config")]
    async fn get_config(&self, _: Context) -> HttpResponse {
        HttpResponse::Ok().data(&self.config)
    }
}

#[sword::main]
async fn main() {
    let in_memory_db = InMemoryDatabase::new();

    let app = Application::builder()?
        .with_state(in_memory_db)?
        .with_controller::<AppController>()
        .build();

    app.run().await?;
}
