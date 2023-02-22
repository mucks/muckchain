use std::sync::Arc;

use crate::{
    core::Encodable,
    crypto::{PublicKey, Signature},
    model::MyHash,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::{sync::RwLock, time::Instant};

use super::{BlockHeader, DynEncoder, Hasher, Transaction, TxHasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,

    pub validator: Option<PublicKey>,
    pub signature: Option<Signature>,

    #[serde(skip)]
    // we cache the hash of the transaction to avoid recomputing it
    hash: Option<MyHash>,
}

#[typetag::serde]
impl Encodable for Block {}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header,
            transactions,
            hash: None,
            validator: None,
            signature: None,
        }
    }

    pub fn from_prev_header(
        prev_header: &BlockHeader,
        header_hasher: &dyn Hasher<BlockHeader>,
        transactions: Vec<Transaction>,
        encoder: DynEncoder,
    ) -> Result<Self> {
        let header = BlockHeader {
            version: prev_header.version,
            height: prev_header.height + 1,
            timestamp: Instant::now().elapsed().as_nanos(),
            data_hash: data_hash(&transactions, &encoder)?,
            prev_block_header_hash: Some(header_hasher.hash(prev_header)?),
        };

        Ok(Block::new(header, transactions))
    }

    pub async fn hash(&mut self, hasher: Box<dyn Hasher<Self>>) -> Result<MyHash> {
        if let Some(hash) = self.hash {
            Ok(hash)
        } else {
            let hash = hasher.hash(self)?;
            self.hash = Some(hash);
            Ok(hash)
        }
    }

    pub fn encode(&self, encoder: &DynEncoder) -> Result<Vec<u8>> {
        encoder.encode(self)
    }
}

// hash all the transactions in the block
fn data_hash(transactions: &[Transaction], encoder: &DynEncoder) -> Result<MyHash> {
    let mut buf: Vec<u8> = vec![];
    for tx in transactions.iter() {
        let data = tx.encode(encoder)?;
        buf.extend_from_slice(&data);
    }
    let hash = Sha256::digest(buf.as_slice());
    Ok(MyHash::from_bytes(hash.as_slice()))
}

// TODO: find a way to include a secret message in the block
pub fn create_genesis_block() -> Block {
    Block::new(
        BlockHeader {
            version: 1,
            height: 0,
            timestamp: tokio::time::Instant::now().elapsed().as_nanos(),
            prev_block_header_hash: None,
            data_hash: MyHash::zero(),
        },
        vec![],
    )
}
