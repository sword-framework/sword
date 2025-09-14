use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;

pub type InMemoryDb = Arc<RwLock<Vec<String>>>;

static IN_MEMORY_DB: LazyLock<InMemoryDb> =
    LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));

#[derive(Clone)]
pub struct AppState {
    pub db: InMemoryDb,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: IN_MEMORY_DB.clone(),
        }
    }
}
