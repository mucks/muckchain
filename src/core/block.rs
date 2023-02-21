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

use super::{BlockHeader, BlockHeaderHasher, DynEncoder, Transaction, TxHasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,

    pub validator: Option<PublicKey>,
    pub signature: Option<Signature>,

    #[serde(skip)]
    // we cache the hash of the transaction to avoid recomputing it
    hash: Arc<RwLock<Option<MyHash>>>,
}

#[typetag::serde]
impl Encodable for Block {}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header,
            transactions,
            hash: Arc::new(RwLock::new(None)),
            validator: None,
            signature: None,
        }
    }

    pub fn from_prev_header(
        prev_header: &BlockHeader,
        transactions: Vec<Transaction>,
        encoder: DynEncoder,
    ) -> Result<Self> {
        let header = BlockHeader {
            version: prev_header.version,
            height: prev_header.height + 1,
            timestamp: Instant::now().elapsed().as_nanos(),
            data_hash: data_hash(&transactions, encoder.clone())?,
            prev_block_hash: Some(
                prev_header.hash(Box::new(BlockHeaderHasher::new(encoder.clone())))?,
            ),
        };

        Ok(Block::new(header, transactions))
    }

    // pub fn hash(&self, encoder: &dyn Encoder) -> Result<MyHash> {
    //     self.header.hash(encoder)
    // }
}

fn data_hash(transactions: &Vec<Transaction>, encoder: DynEncoder) -> Result<MyHash> {
    let mut buf: Vec<u8> = vec![];
    for tx in transactions.iter() {
        let data = tx.encode(encoder.clone())?;
        buf.extend_from_slice(&data);
    }
    let hash = Sha256::digest(buf.as_slice());
    Ok(MyHash::from_bytes(hash.as_slice()))
}
