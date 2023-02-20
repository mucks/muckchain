use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::model::MyHash;

use super::{DynEncoder, Hasher, TxHasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub data: Vec<u8>,
    #[serde(skip)]
    pub hash: Option<MyHash>,
}

impl Transaction {
    pub fn new(data: Vec<u8>) -> Self {
        Transaction { data, hash: None }
    }

    /*
        TODO: cache this inside the transaction, maybe with rwlock
            since this is an expensive operation
    */
    pub fn hash(&self, encoder: DynEncoder) -> Result<()> {
        let hash = TxHasher.hash(encoder, self)?;
        Ok(())
    }

    pub fn encode(&self, encoder: DynEncoder) -> Result<Vec<u8>> {
        encoder.encode(self)
    }
}
