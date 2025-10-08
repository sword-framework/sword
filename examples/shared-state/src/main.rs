use serde_json::json;
use std::sync::Arc;
use sword::prelude::*;
use tokio::sync::RwLock;

pub type InMemoryDb = Arc<RwLock<Vec<String>>>;

#[controller("/api")]
struct AppController {
    db: InMemoryDb,
    config: Config,
}

#[routes]
impl AppController {
    #[get("/data")]
    async fn submit_data(&self) -> HttpResult<HttpResponse> {
        let count = self.db.read().await.len();
        let message = format!("Current data count: {count}");

        self.db.write().await.push(message);

        Ok(HttpResponse::Ok().data(json!({
            "count": count,
        })))
    }

    #[get("/config")]
    async fn get_config(&self) -> HttpResult<HttpResponse> {
        let value = self.config.get::<ApplicationConfig>().unwrap_or_default();
        Ok(HttpResponse::Ok().data(&value))
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()
        .with_state(Arc::new(RwLock::new(Vec::<String>::new())))
        .with_controller::<AppController>()
        .build();

    app.run().await;
}
