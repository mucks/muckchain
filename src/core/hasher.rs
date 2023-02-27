use std::fmt::Debug;

use super::{Block, BlockHeader, DynEncoder, Transaction};
use crate::model::MyHash;
use anyhow::Result;
use dyn_clone::DynClone;
use sha2::{Digest, Sha256};

pub type DynHasher<T> = Box<dyn Hasher<T>>;

pub trait Hasher<T>: Debug + DynClone + Send + Sync
where
    T: Sized,
{
    fn hash(&self, t: &T) -> Result<MyHash>;
}

dyn_clone::clone_trait_object!(Hasher<Transaction>);
dyn_clone::clone_trait_object!(Hasher<Block>);
dyn_clone::clone_trait_object!(Hasher<BlockHeader>);

#[derive(Debug, Clone)]
pub struct BlockHasher {
    enc: DynEncoder,
}

impl BlockHasher {
    pub fn new(enc: DynEncoder) -> Self {
        BlockHasher { enc }
    }
}

impl Hasher<Block> for BlockHasher {
    fn hash(&self, block: &Block) -> Result<MyHash> {
        let bytes = block.header.bytes(&self.enc)?;
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}

impl Hasher<BlockHeader> for BlockHasher {
    fn hash(&self, block_header: &BlockHeader) -> Result<MyHash> {
        let bytes = block_header.bytes(&self.enc)?;
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}

#[derive(Debug, Clone)]
pub struct TxHasher;

impl Hasher<Transaction> for TxHasher {
    // currently the encoder is not needed here as we only hash the transaction data
    fn hash(&self, tx: &Transaction) -> Result<MyHash> {
        let bytes = tx.data.clone();
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}
