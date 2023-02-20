use super::{BlockHeader, DynEncoder, JsonEncoder, Transaction};
use crate::model::MyHash;
use anyhow::Result;
use sha2::{Digest, Sha256};

pub trait Hasher<T>
where
    T: Sized,
{
    fn hash(&self, encoder: DynEncoder, t: &T) -> Result<MyHash>;
}

pub struct BlockHeaderHasher;

impl Hasher<BlockHeader> for BlockHeaderHasher {
    fn hash(&self, enc: DynEncoder, block_header: &BlockHeader) -> Result<MyHash> {
        let bytes = block_header.bytes(enc)?;
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}

pub struct TxHasher;

impl Hasher<Transaction> for TxHasher {
    fn hash(&self, enc: DynEncoder, tx: &Transaction) -> Result<MyHash> {
        let bytes = tx.data.clone();
        let hash = MyHash::from_bytes(Sha256::digest(bytes).as_slice());
        Ok(hash)
    }
}
