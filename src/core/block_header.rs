use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub height: u32,
    pub timestamp: u128,
    pub data_hash: Hash,
    pub prev_block_header_hash: Option<Hash>,
}

#[typetag::serde]
impl Encodable for BlockHeader {}

impl BlockHeader {
    pub fn bytes(&self, enc: &DynEncoder) -> Result<Vec<u8>> {
        enc.encode(self)
    }
    pub fn hash(&self, hasher: &DynHasher<Self>) -> Result<Hash> {
        hasher.hash(self)
    }
}
