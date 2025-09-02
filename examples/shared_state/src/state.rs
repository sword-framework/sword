use std::sync::{Arc, OnceLock};

use tokio::sync::RwLock;

pub type InMemoryDb = Arc<RwLock<Vec<String>>>;

static IN_MEMORY_DB: OnceLock<InMemoryDb> = OnceLock::new();

pub fn db() -> Arc<RwLock<Vec<String>>> {
    IN_MEMORY_DB.get_or_init(|| Arc::new(RwLock::new(Vec::new()))).clone()
}

#[derive(Clone)]
pub struct AppState {
    pub db: InMemoryDb,
}

impl AppState {
    pub fn new() -> Self {
        Self { db: db() }
    }
}
