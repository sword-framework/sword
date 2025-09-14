mod controller;
mod database;
mod logger;

use shaku::module;
use sword::prelude::*;

use crate::database::InMemoryDatabase;
use crate::logger::ConsoleLogger;
use controller::UserController;

module! {
    pub AppModule {
        components = [ConsoleLogger, InMemoryDatabase],
        providers = []
    }
}

#[sword::main]
async fn main() {
    let module = AppModule::builder().build();

    let app = Application::builder()?
        .with_shaku_di_module(module)?
        .with_controller::<UserController>()
        .build();

    app.run().await?;
}
