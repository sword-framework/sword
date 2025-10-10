use crate::database::Database;
use serde_json::Value;

use sword::core::Injectable;

pub struct TaskRepository {
    db: Database,
}

impl TaskRepository {
    pub async fn create(&self, task: Value) {
        self.db.insert("tasks", task).await;
    }

    pub async fn find_all(&self) -> Option<Vec<Value>> {
        self.db.get_all("tasks").await
    }
}

impl Clone for TaskRepository {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
        }
    }
}

impl Injectable for TaskRepository {
    fn build(state: &sword::core::State) -> Self {
        let db = state
            .get::<Database>()
            .expect("Database should be registered in the state");

        Self { db }
    }

    fn dependencies() -> Vec<std::any::TypeId> {
        vec![std::any::TypeId::of::<Database>()]
    }
}
