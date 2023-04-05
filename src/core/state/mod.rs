pub mod mem_state;

use anyhow::Result;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub type DynState = Box<dyn State>;

#[async_trait::async_trait]
pub trait State: Debug + DynClone + Send + Sync {
    async fn set(&self, key: &[u8], value: &[u8]) -> Result<()>;
    async fn get(&self, key: &[u8]) -> Result<Vec<u8>>;
    async fn delete(&self, key: &[u8]) -> Result<()>;
}

dyn_clone::clone_trait_object!(State);
