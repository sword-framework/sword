use serde_json::json;
use std::sync::{Arc, OnceLock};
use sword::prelude::*;
use tokio::sync::RwLock;

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

struct MyMiddleware;

impl MiddlewareWithState<AppState> for MyMiddleware {
    async fn handle(ctx: State<AppState>, mut req: Request, next: Next) -> MiddlewareResult {
        let count = ctx.db.read().await.len();
        req.extensions.insert(count);

        Ok(next.run(req.into()).await)
    }
}

#[controller("/api")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/data")]
    #[middleware(MyMiddleware)]
    async fn submit_data(state: State<AppState>, req: Request) -> HttpResponse {
        let db = &state.db;
        let count = req.extensions.get::<usize>().cloned().unwrap_or(0);
        let message = format!("Current data count: {}", count);

        db.write().await.push(message);

        HttpResponse::Ok().data(json!({
            "count": count,
            "current_data": db.read().await.clone(),
        }))
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
