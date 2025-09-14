use serde_json::json;

use sword::prelude::*;
use sword::web::HttpResult;

mod state;
use crate::state::AppState;

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

#[sword::main]
async fn main() {
    let app_state = AppState::new();

    let app = Application::builder()?
        .with_state(app_state)?
        .with_controller::<AppController>()
        .build();

    app.run().await?;
}
