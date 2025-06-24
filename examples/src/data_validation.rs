use serde::{Deserialize, Serialize};
use sword::{
    application::Application,
    controller::{controller, controller_impl},
    http::{Context, HttpResponse, RequestMethods, Result},
    routing::{get, post},
};

use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
struct MyQuery {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    name: String,
}

#[derive(Serialize, Deserialize, Validate)]
struct MyBody {
    #[validate(length(min = 1, message = "Content must not be empty"))]
    content: String,
}

#[controller("/")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/hello")]
    async fn hello(ctx: Context) -> Result<HttpResponse> {
        let query = ctx.validated_query::<MyQuery>()?;
        Ok(HttpResponse::Ok().data(query))
    }

    #[post("/submit")]
    async fn submit_data(ctx: Context) -> Result<HttpResponse> {
        let body = ctx.validated_body::<MyBody>()?;

        Ok(HttpResponse::Ok()
            .data(body)
            .message("Data submitted successfully"))
    }
}

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<AppController>()
        .run()
        .await;
}
