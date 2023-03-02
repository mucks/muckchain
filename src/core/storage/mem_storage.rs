use std::{collections::HashMap, sync::Arc};

use super::Storage;

use async_trait::async_trait;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct MemStorage {
    data: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl MemStorage {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// TODO: might be optimized with using readers and writers  instead of vec<u8>

#[async_trait]
impl Storage for MemStorage {
    async fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let map = self.data.read().await;
        map.get(key).cloned()
    }

    async fn put(&self, key: &[u8], value: &[u8]) {
        let mut map = self.data.write().await;
        let entry = map.entry(key.to_vec());
        entry.or_default().extend_from_slice(value);
    }

    async fn delete(&self, key: &[u8]) {
        let mut map = self.data.write().await;
        map.remove(key);
    }
}
