use std::sync::Arc;

use shaku::{module, Component, Interface};
use sword::{
    di::Inject,
    http::HttpResponse,
    prelude::{controller, controller_impl, Application},
};

use sword_macros::get;

trait Logger: Interface {
    fn log(&self, message: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}

module! {
    AppModule {
        components = [ConsoleLogger],
        providers = []
    }
}

#[controller("/users")]
struct UserController {}

#[controller_impl]
impl UserController {
    #[get("/")]
    async fn get_users(logger: Inject<AppModule, dyn Logger>) -> HttpResponse {
        logger.log("Fetching users");
        HttpResponse::Ok().data("Users fetched successfully")
    }
}

#[tokio::main]
async fn main() {
    let module = AppModule::builder().build();

    Application::builder()
        .di_module(Arc::new(module))
        .controller::<UserController>()
        .run("0.0.0.0:8080")
        .await;
}
