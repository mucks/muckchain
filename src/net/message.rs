use super::rpc::RPC;
use crate::prelude::*;
use std::ops::Range;

//TODO: handle the the large size difference in this enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Transaction(Transaction),
    Text(String),
    Block(Block),
    GetStatus,
    Status(Status),
    GetBlocks(Range<u32>),
    Blocks(Vec<Block>),
}

#[typetag::serde]
impl Decodable for Message {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[typetag::serde]
impl Encodable for Message {}

impl Message {
    pub fn from_rpc(decoder: &DynDecoder, rpc: &RPC) -> Result<Self> {
        let msg = decode(decoder, &rpc.data)?;
        Ok(msg)
    }
    pub fn bytes(&self, encoder: &DynEncoder) -> Result<Vec<u8>> {
        encoder.encode(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub height: u32,
}

#[typetag::serde]
impl Encodable for Status {}
