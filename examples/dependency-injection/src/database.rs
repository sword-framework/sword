use std::{collections::HashMap, sync::Arc};

use serde_json::Value;
use sword::prelude::*;
use tokio::sync::RwLock;

pub type Store = Arc<RwLock<HashMap<&'static str, Vec<Value>>>>;

#[provider]
pub struct Database {
    db: Store,
}

impl Database {
    pub async fn new() -> Self {
        let db = Arc::new(RwLock::new(HashMap::new()));

        db.write().await.insert("tasks", Vec::new());

        Self { db }
    }

    pub async fn insert(&self, table: &'static str, record: Value) {
        let mut db = self.db.write().await;

        if let Some(table_data) = db.get_mut(table) {
            table_data.push(record);
        }
    }

    pub async fn get_all(&self, table: &'static str) -> Option<Vec<Value>> {
        let db = self.db.read().await;

        db.get(table).cloned()
    }
}
