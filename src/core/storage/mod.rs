pub mod mem_storage;

use async_trait::async_trait;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub type DynStorage = Box<dyn Storage>;

// TODO: maybe implement a another function to get a range of keys
#[async_trait]
pub trait Storage: Debug + DynClone + Send + Sync {
    async fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    async fn put(&self, key: &[u8], value: &[u8]);
    async fn delete(&self, key: &[u8]);
}
dyn_clone::clone_trait_object!(Storage);
