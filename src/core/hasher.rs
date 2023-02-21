use super::{BlockHeader, DynEncoder, JsonEncoder, Transaction};
use crate::model::MyHash;
use anyhow::Result;
use sha2::{Digest, Sha256};

pub trait Hasher<T>
where
    T: Sized,
{
    fn hash(&self, t: &T) -> Result<MyHash>;
}

pub struct BlockHeaderHasher {
    enc: DynEncoder,
}
impl BlockHeaderHasher {
    pub fn new(enc: DynEncoder) -> Self {
        BlockHeaderHasher { enc }
    }
}

impl Hasher<BlockHeader> for BlockHeaderHasher {
    fn hash(&self, block_header: &BlockHeader) -> Result<MyHash> {
        let bytes = block_header.bytes(&self.enc)?;
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}

pub struct TxHasher;

impl Hasher<Transaction> for TxHasher {
    // currently the encoder is not needed here as we only hash the transaction data
    fn hash(&self, tx: &Transaction) -> Result<MyHash> {
        let bytes = tx.data.clone();
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}
