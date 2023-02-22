use std::fmt::Debug;

use super::{Block, BlockHeader, DynEncoder, Transaction};
use crate::model::MyHash;
use anyhow::Result;
use sha2::{Digest, Sha256};

pub trait Hasher<T>: HasherClone<T> + Debug + Send + Sync
where
    T: Sized,
{
    fn hash(&self, t: &T) -> Result<MyHash>;
}

pub trait HasherClone<U> {
    fn clone_box(&self) -> Box<dyn Hasher<U>>;
}

impl<T, U> HasherClone<U> for T
where
    T: 'static + Hasher<T> + Clone + Hasher<U>,
{
    fn clone_box(&self) -> Box<dyn Hasher<U>> {
        Box::new(self.clone())
    }
}

impl<U> Clone for Box<dyn Hasher<U>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct BlockHasher {
    enc: DynEncoder,
}

impl HasherClone<Block> for BlockHasher {
    fn clone_box(&self) -> Box<dyn Hasher<Block>> {
        Box::new(self.clone())
    }
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
impl HasherClone<BlockHeader> for BlockHasher {
    fn clone_box(&self) -> Box<dyn Hasher<BlockHeader>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TxHasher;

impl HasherClone<Transaction> for TxHasher {
    fn clone_box(&self) -> Box<dyn Hasher<Transaction>> {
        Box::new(self.clone())
    }
}

impl Hasher<Transaction> for TxHasher {
    // currently the encoder is not needed here as we only hash the transaction data
    fn hash(&self, tx: &Transaction) -> Result<MyHash> {
        let bytes = tx.data.clone();
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}
