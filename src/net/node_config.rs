use crate::core::{DynDecoder, DynEncoder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub encoder: DynEncoder,
    pub decoder: Arc<Mutex<DynDecoder>>,
}
