use sword::{
    application::Application,
    controller::{controller, controller_impl},
    http::{Context, HttpResponse, Result},
    middleware::{middleware, Middleware, MiddlewareHandler, MiddlewareResult, NextFunction},
    routing::get,
};

#[derive(Middleware)]
struct MyMiddleware;

impl MiddlewareHandler for MyMiddleware {
    async fn handle(mut ctx: Context, next: NextFunction) -> MiddlewareResult {
        ctx.extensions
            .insert("Middleware executed successfully".to_string());

        Ok(next.run(ctx).await)
    }
}

#[controller("/api")]
struct AppController {}

#[controller_impl]
impl AppController {
    #[get("/data")]
    #[middleware(MyMiddleware)]
    async fn submit_data(ctx: Context) -> Result<HttpResponse> {
        let mw_message = ctx.extensions.get::<String>().unwrap();
        Ok(HttpResponse::Ok().message(mw_message))
    }
}

#[tokio::main]
async fn main() {
    Application::builder()
        .controller::<AppController>()
        .run()
        .await;
}
