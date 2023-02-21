use super::{DynEncoder, Encodable, Hasher};
use crate::model::MyHash;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub height: u32,
    pub timestamp: u128,
    pub data_hash: MyHash,
    pub prev_block_hash: Option<MyHash>,
}

#[typetag::serde]
impl Encodable for BlockHeader {}

impl BlockHeader {
    pub fn bytes(&self, enc: &DynEncoder) -> Result<Vec<u8>> {
        enc.encode(self)
    }

    pub fn hash(&self, hasher: Box<dyn Hasher<Self>>) -> Result<MyHash> {
        hasher.hash(self)
    }
}
