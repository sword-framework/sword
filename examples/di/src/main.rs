mod database;
mod repository;
mod service;

pub use repository::TaskRepository;

use serde_json::json;
use sword::{core::DependencyContainer, prelude::*};

use crate::{database::Database, service::TasksService};

#[controller("/tasks", version = "v1")]
struct TasksController {
    tasks: TasksService,
}

#[routes]
impl TasksController {
    #[get("/")]
    async fn get_tasks(&self) -> HttpResponse {
        let data = self.tasks.find_all().await;

        HttpResponse::Ok().data(data)
    }

    #[post("/")]
    async fn create_task(&self) -> HttpResponse {
        let total_task = self.tasks.find_all().await.len();

        let task = json!({
            "id": total_task + 1,
            "title": format!("Task {}", total_task + 1),
        });

        self.tasks.create(task.clone()).await;

        HttpResponse::Created().message("Task created").data(task)
    }
}

#[sword::main]
async fn main() {
    let db = Database::new().await;

    let container = DependencyContainer::builder()
        .register::<TaskRepository>()
        .register::<TasksService>()
        .register_instance(db)
        .build();

    let app = Application::builder()
        .with_dependency_container(container)
        .with_controller::<TasksController>()
        .build();

    app.run().await;
}
