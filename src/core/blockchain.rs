use crate::{
    model::MyHash,
    net::{EncodingConfig, HasherConfig},
};

use super::{
    block_header::BlockHeader, Block, DynBlockValidator, DynEncoder, DynHasher, DynStorage, Hasher,
};
use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    pub genesis_block: Block,
    pub storage: DynStorage,
    pub block_validator: DynBlockValidator,
    pub hashers: HasherConfig,
    pub encoding: EncodingConfig,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub config: BlockchainConfig,
    block_headers: Arc<RwLock<Vec<BlockHeader>>>,
}

impl Blockchain {
    pub fn new(config: BlockchainConfig) -> Self {
        Self {
            block_headers: Arc::new(RwLock::new(vec![config.genesis_block.header.clone()])),
            config,
        }
    }

    pub async fn add_block(&self, mut block: Block) -> Result<()> {
        // Validate the block
        self.config
            .block_validator
            .validate(self, &mut block, &self.config.hashers.block_hasher)
            .await?;

        // Execute all transactions
        for tx in block.transactions.iter() {
            // Execute the transaction
        }

        // Add the block
        self.block_headers.write().await.push(block.header);
        Ok(())
    }

    pub async fn has_block(&self, height: u32) -> bool {
        self.block_headers.read().await.len() > height as usize
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

    // pub async fn get_prev_header_hash(&self, height: u32) -> Result<MyHash> {
    //     let prev_header = self.get_prev_header(height).await.ok_or_else(|| {
    //         anyhow!(
    //             "could not get previous block header for block_height: {}",
    //             height
    //         )
    //     })?;
    //     self.block_hasher.hash(&prev_header)
    // }

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
