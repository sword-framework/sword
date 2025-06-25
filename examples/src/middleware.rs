// use sword::prelude::*;

// struct MyMiddleware;

// impl Middleware for MyMiddleware {
//     async fn handle(mut req: Request, next: Next) -> MiddlewareResult {
//         req.extensions
//             .insert("Middleware executed successfully".to_string());

//         Ok(next.run(req.into()).await)
//     }
// }

// #[controller("/api")]
// struct AppController {}

// #[controller_impl]
// impl AppController {
//     #[get("/data")]
//     #[middleware(MyMiddleware)]
//     async fn submit_data(req: Request) -> Result<HttpResponse> {
//         let mw_message = req.extensions.get::<String>().unwrap();
//         Ok(HttpResponse::Ok().message(mw_message))
//     }
// }

use sword::prelude::Application;

#[tokio::main]
async fn main() {
    Application::builder()
        // .controller::<AppController>()
        .run()
        .await;
}
