use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::models::Task;

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
}

impl AppState {
    pub fn new() -> AppState {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}