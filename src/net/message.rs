use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::core::{decode, Block, Decodable, DynDecoder, DynEncoder, Encodable, Transaction};

use super::rpc::RPC;

//TODO: handle the the large size difference in this enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Transaction(Transaction),
    Text(String),
    Block(Block),
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
    pub fn from_rpc(decoder: DynDecoder, rpc: &RPC) -> Result<Self> {
        let msg = decode(decoder, &rpc.data)?;
        Ok(msg)
    }
    pub fn bytes(&self, encoder: &DynEncoder) -> Result<Vec<u8>> {
        encoder.encode(self)
    }
}
