use std::sync::{Arc, RwLock};
use sword::core::TryFromState;

#[derive(Clone, TryFromState)]
pub struct InMemoryDatabase {
    data: Arc<RwLock<Vec<String>>>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn read(&self) -> Vec<String> {
        self.data.read().unwrap().clone()
    }

    pub fn write(&self, name: String) {
        self.data.write().unwrap().push(name);
    }
}
