use serde_json::json;
use std::sync::{Arc, OnceLock};
use sword::prelude::*;
use tokio::sync::RwLock;

use sword::http::Result;

type InMemoryDb = Arc<RwLock<Vec<String>>>;
const IN_MEMORY_DB: OnceLock<InMemoryDb> = OnceLock::new();

fn db() -> Arc<RwLock<Vec<String>>> {
    IN_MEMORY_DB
        .get_or_init(|| Arc::new(RwLock::new(Vec::new())))
        .clone()
}

#[derive(Clone)]
struct AppState {
    db: InMemoryDb,
}

#[controller("/api")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/data")]
    async fn submit_data(ctx: Context) -> Result<HttpResponse> {
        let state = ctx.get_state::<AppState>()?;

        let count = state.db.read().await.len();
        let message = format!("Current data count: {}", count);

        state.db.write().await.push(message);

        Ok(HttpResponse::Ok().data(json!({
            "count": count,
            "current_data": state.db.read().await.clone(),
        })))
    }
}

#[tokio::main]
async fn main() {
    let app_state = AppState { db: db() };

    Application::builder()
        .state(app_state)
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await;
}
