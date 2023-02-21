use std::sync::Arc;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    core::Encodable,
    crypto::{PublicKey, Signature},
    model::MyHash,
};

use super::{Decodable, DynEncoder, Hasher, TxHasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub data: Vec<u8>,

    public_key_of_sender: Option<PublicKey>,
    signature: Option<Signature>,

    #[serde(skip)]
    // we cache the hash of the transaction to avoid recomputing it
    hash: Arc<RwLock<Option<MyHash>>>,
    #[serde(skip)]
    first_seen: u128,
}

#[typetag::serde]
impl Encodable for Transaction {}

#[typetag::serde]
impl Decodable for Transaction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

//TODO: add method to sign the transaction
impl Transaction {
    pub fn new(data: Vec<u8>) -> Self {
        Transaction {
            data,
            hash: Arc::new(RwLock::new(None)),
            first_seen: 0,
            public_key_of_sender: None,
            signature: None,
        }
    }

    pub fn verify(&self) -> Result<()> {
        let sig = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow!("transaction has no signature!"))?;

        let pub_key = self
            .public_key_of_sender
            .as_ref()
            .ok_or_else(|| anyhow!("transaction has no public_key_of_sender!"))?;

        if sig.verify(&self.data, pub_key) {
            Ok(())
        } else {
            Err(anyhow!("transaction has an invalid signature!"))
        }
    }

    pub fn first_seen(&self) -> u128 {
        self.first_seen
    }

    // this creates an invalid state optimize it to be done automatically
    pub fn set_first_seen(&mut self, first_seen: u128) {
        self.first_seen = first_seen;
    }

    pub async fn hash(&self, encoder: DynEncoder) -> Result<MyHash> {
        if let Some(hash) = self.hash.read().await.as_ref() {
            Ok(*hash)
        } else {
            let hash = TxHasher.hash(self)?;
            *self.hash.write().await = Some(hash);
            Ok(hash)
        }
    }

    pub fn encode(&self, encoder: DynEncoder) -> Result<Vec<u8>> {
        encoder.encode(self)
    }
}
