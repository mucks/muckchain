use super::Hasher;
use super::{Block, Blockchain, DynHasher};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub type DynBlockValidator = Box<dyn BlockValidator>;

#[async_trait]
pub trait BlockValidator: Send + Sync + Debug + DynClone {
    async fn validate(
        &self,
        bc: &Blockchain,
        block: &mut Block,
        hasher: &DynHasher<Block>,
    ) -> Result<()>;
}
dyn_clone::clone_trait_object!(BlockValidator);

#[derive(Debug, Clone)]
pub struct DefaultBlockValidator {}

#[async_trait]
impl BlockValidator for DefaultBlockValidator {
    async fn validate(
        &self,
        bc: &Blockchain,
        block: &mut Block,
        hasher: &DynHasher<Block>,
    ) -> Result<()> {
        // Check if block already exists
        if bc.has_block(block.header.height).await {
            let hash = block.hash(hasher)?;
            return Err(anyhow::anyhow!("block {hash} already exists"));
        }

        // Check if block height is too high
        let bc_height = bc.height().await;
        if block.header.height != bc_height + 1 {
            return Err(anyhow!("block height {} is too high", block.header.height));
        }

        // Check if block is valid

        let prev_header = bc
            .get_prev_header(block.header.height)
            .await
            .ok_or_else(|| anyhow!("bc (height:{bc_height}) has no previous block!"))?;

        let prev_header_hash = Block::hash_from_header(&prev_header, hasher)?;

        if block.header.prev_block_header_hash != Some(prev_header_hash) {
            return Err(anyhow!("invalid block"));
        }

        block.verify(&bc.config.encoding.encoder)?;

        Ok(())
    }
}
