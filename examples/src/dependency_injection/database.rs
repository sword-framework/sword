use async_trait::async_trait;
use std::sync::Arc;

use shaku::{Component, Interface};
use tokio::sync::RwLock;

#[async_trait]
pub trait DataRepository: Interface {
    async fn get_data(&self) -> Vec<String>;
    async fn add_data(&self, data: String);
}

#[derive(Component)]
#[shaku(interface = DataRepository)]
pub struct InMemoryDatabase {
    #[shaku(default)]
    data: Arc<RwLock<Vec<String>>>,
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl DataRepository for InMemoryDatabase {
    async fn add_data(&self, data: String) {
        self.data.write().await.push(data);
    }

    async fn get_data(&self) -> Vec<String> {
        self.data.read().await.clone()
    }
}
