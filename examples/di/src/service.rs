use crate::TaskRepository;
use serde_json::Value;
use sword::core::Injectable;

pub struct TasksService {
    repository: TaskRepository,
}

impl TasksService {
    pub async fn create(&self, task: Value) {
        self.repository.create(task).await;
    }

    pub async fn find_all(&self) -> Vec<Value> {
        self.repository.find_all().await.unwrap_or_default()
    }
}

impl Clone for TasksService {
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
        }
    }
}

impl Injectable for TasksService {
    fn build(state: &sword::core::State) -> Self {
        let repository = state
            .get::<TaskRepository>()
            .expect("TaskRepository should be registered in the state");

        Self { repository }
    }

    fn dependencies() -> Vec<std::any::TypeId> {
        vec![std::any::TypeId::of::<TaskRepository>()]
    }
}
