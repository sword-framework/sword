mod controller;
mod database;
mod logger;

use shaku::module;
use sword::application::Application;

use crate::database::InMemoryDatabase;
use crate::logger::ConsoleLogger;
use controller::UserController;

module! {
    pub AppModule {
        components = [ConsoleLogger, InMemoryDatabase],
        providers = []
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let module = AppModule::builder().build();

    Application::builder()?
        .state(module)?
        .controller::<UserController>()
        .run()
        .await?;

    Ok(())
}
