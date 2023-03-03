use crate::{
    config::{EncodingConfig, HasherConfig},
    prelude::*,
};

use super::{
    block_header::BlockHeader, state::DynState, storage::DynStorage, vm::DynVM, Block,
    DynBlockValidator,
};
use std::{ops::Range, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    pub genesis_block: Block,
    pub storage: DynStorage,
    pub block_validator: DynBlockValidator,
    pub hashers: HasherConfig,
    pub encoding: EncodingConfig,
    pub vm: DynVM,
    pub state: DynState,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub config: BlockchainConfig,
    block_headers: Arc<RwLock<Vec<BlockHeader>>>,
}

impl Blockchain {
    pub async fn new(config: BlockchainConfig) -> Result<Self> {
        let genesis_block = config.genesis_block.clone();

        let bc = Self {
            block_headers: Arc::new(RwLock::new(vec![])),
            config,
        };

        bc.add_block_without_validation(genesis_block).await?;

        Ok(bc)
    }

    pub async fn add_block(&self, mut block: Block) -> Result<()> {
        // Validate the block
        self.config
            .block_validator
            .validate(self, &mut block, &self.config.hashers.block_hasher)
            .await?;

        // TODO: implement execution of transaction code
        for tx in block.transactions.iter() {}

        self.add_block_without_validation(block).await?;

        // Add the block
        Ok(())
    }

    async fn add_block_without_validation(&self, block: Block) -> Result<()> {
        self.block_headers.write().await.push(block.header.clone());
        self.save_block(block).await
    }

    pub async fn has_block(&self, height: u32) -> bool {
        self.block_headers.read().await.len() > height as usize
    }

    pub async fn get_header(&self, height: u32) -> Option<BlockHeader> {
        let block_headers = self.block_headers.read().await;
        let block_header = block_headers.get(height as usize)?;
        Some(block_header.clone())
    }

    pub async fn len(&self) -> usize {
        self.block_headers.read().await.len()
    }

    // Get a range of blocks
    pub async fn get_blocks(&self, range: Range<u32>) -> Result<Vec<Block>> {
        // Since we're passing a range we have to use len here
        let len = self.len().await as u32;

        if range.end > len {
            return Err(anyhow!("range end is greater than blockchain height"));
        }

        let mut blocks = vec![];
        for height in range {
            let block = self.get_block(height).await?;
            blocks.push(block);
        }

        Ok(blocks)
    }

    pub async fn get_block(&self, height: u32) -> Result<Block> {
        let bytes = self
            .config
            .storage
            .get(&height.to_le_bytes())
            .await
            .ok_or_else(|| anyhow!("could not get block from storage"))?;

        let block = Block::decode(&bytes, &self.config.encoding.decoder)?;

        Ok(block)
    }

    async fn save_block(&self, block: Block) -> Result<()> {
        let bytes = block.encode(&self.config.encoding.encoder)?;
        self.config
            .storage
            .put(&block.header.height.to_le_bytes(), &bytes)
            .await;

        Ok(())
    }

    pub async fn height(&self) -> u32 {
        self.block_headers.read().await.len() as u32 - 1
    }

    // Get the previous block header
    pub async fn get_prev_header(&self, height: u32) -> Option<BlockHeader> {
        if height == 0 {
            self.get_header(0).await
        } else {
            self.get_header(height - 1).await
        }
    }

    // pub async fn get_prev_header_hash(&self, height: u32) -> Result<Hash> {
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
    // ) -> Result<Hash> {
    //     let prev_header = self.get_prev_header(height).await.ok_or_else(|| {
    //         anyhow!(
    //             "could not get previous block header for block_height: {}",
    //             height
    //         )
    //     })?;
    //     hasher.hash(&prev_header)
    // }
}
