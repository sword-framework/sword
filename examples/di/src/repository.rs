use crate::database::Database;
use serde_json::Value;

use sword::core::injectable;

#[injectable]
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
