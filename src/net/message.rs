use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::core::{DynDecoder, Transaction};

use super::rpc::RPC;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Transaction(Transaction),
}

impl Message {
    pub fn from_rpc(rpc: &RPC, decoder: DynDecoder) -> Result<Self> {
        // let mut dec = *decoder.lock().unwrap();
        // let msg: Message = from_decoder(&mut dec, &rpc.data)?;
        todo!()
        // let msg: Message = from_decoder(decoder, &rpc.data)?;
        // Ok(msg)
    }
}
