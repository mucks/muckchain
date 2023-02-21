use super::{block_header::BlockHeader, Block};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Blockchain {
    block_headers: Arc<RwLock<Vec<BlockHeader>>>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            block_headers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    // TODO: Verify the block here
    pub async fn add_block(&self, block: Block) {
        self.block_headers.write().await.push(block.header);
    }

    pub async fn get_header(&self, height: u32) -> Option<BlockHeader> {
        let block_headers = self.block_headers.read().await;
        let block_header = block_headers.get(height as usize)?;
        Some(block_header.clone())
    }

    pub async fn height(&self) -> u32 {
        self.block_headers.read().await.len() as u32 - 1
    }

    pub async fn get_prev_header(&self, height: u32) -> Option<BlockHeader> {
        if height == 0 {
            return None;
        }

        let block_headers = self.block_headers.read().await;
        let block_header = block_headers.get((height - 1) as usize)?;
        Some(block_header.clone())
    }
}
