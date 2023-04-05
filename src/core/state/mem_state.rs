use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use super::State;

#[derive(Debug, Clone)]
pub struct MemState {
    state: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl MemState {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl State for MemState {
    async fn set(&self, key: &[u8], value: &[u8]) -> anyhow::Result<()> {
        let mut state = self.state.write().await;
        state.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    async fn get(&self, key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let state = self.state.read().await;
        println!("state: {:?}", state);
        Ok(state
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("state could not find key: {:?}", key))?
            .to_vec())
    }

    async fn delete(&self, key: &[u8]) -> anyhow::Result<()> {
        let mut state = self.state.write().await;
        state.remove(key);
        Ok(())
    }
}
