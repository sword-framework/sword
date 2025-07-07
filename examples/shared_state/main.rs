use serde_json::json;

use sword::prelude::*;
use sword::web::HttpResult;

mod state;

use crate::state::{db, AppState};

#[controller("/api")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/data")]
    async fn submit_data(ctx: Context) -> HttpResult<HttpResponse> {
        let state = ctx.get_state::<AppState>()?;

        let count = state.db.read().await.len();
        let message = format!("Current data count: {count}");

        state.db.write().await.push(message);

        Ok(HttpResponse::Ok().data(json!({
            "count": count,
            "current_data": state.db.read().await.clone(),
        })))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState { db: db() };

    Application::builder()
        .state(app_state)?
        .controller::<AppController>()
        .run("0.0.0.0:8080")
        .await?;

    Ok(())
}
