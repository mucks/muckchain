use crate::model::MyHash;

use super::{block_header::BlockHeader, Block, DynStorage, Hasher};
use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    pub genesis_block: Block,
    pub storage: DynStorage,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    config: BlockchainConfig,
    block_headers: Arc<RwLock<Vec<BlockHeader>>>,
}

impl Blockchain {
    pub fn new(config: BlockchainConfig) -> Self {
        Self {
            block_headers: Arc::new(RwLock::new(vec![config.genesis_block.header.clone()])),
            config,
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
            self.get_header(0).await
        } else {
            self.get_header(height - 1).await
        }
    }

    // pub async fn get_prev_block_hash(
    //     &self,
    //     hasher: &dyn Hasher<BlockHeader>,
    //     height: u32,
    // ) -> Result<MyHash> {
    //     let prev_header = self.get_prev_header(height).await.ok_or_else(|| {
    //         anyhow!(
    //             "could not get previous block header for block_height: {}",
    //             height
    //         )
    //     })?;
    //     hasher.hash(&prev_header)
    // }
}
