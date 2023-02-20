use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::core::{decode, Decodable, DynDecoder, Transaction};

use super::rpc::RPC;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Transaction(Transaction),
}

#[typetag::serde]
impl Decodable for Message {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Message {
    pub fn from_rpc(decoder: DynDecoder, rpc: &RPC) -> Result<Self> {
        let msg = decode(decoder, &rpc.data)?;
        Ok(msg)
    }
}
