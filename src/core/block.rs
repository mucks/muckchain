use crate::{
    core::Encodable,
    crypto::{PrivateKey, PublicKey, Signature},
    model::MyHash,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::time::Instant;

use super::{BlockHeader, DynEncoder, DynHasher, Hasher, Transaction, TxHasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,

    // the validator that created this block
    pub validator_public_key: Option<PublicKey>,
    pub signature: Option<Signature>,

    #[serde(skip)]
    // we cache the hash of the transaction to avoid recomputing it
    hash: Option<MyHash>,
}

#[typetag::serde]
impl Encodable for Block {}

impl Block {
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>,
        enc: &DynEncoder,
    ) -> Result<Self> {
        let mut b = Block {
            header,
            transactions,
            hash: None,
            validator_public_key: None,
            signature: None,
        };
        let data_hash = data_hash(&b.transactions, enc)?;
        b.header.data_hash = data_hash;
        Ok(b)
    }

    pub fn hash_from_header(header: &BlockHeader, hasher: &DynHasher<Block>) -> Result<MyHash> {
        let mut b = Block {
            header: header.clone(),
            transactions: vec![],
            hash: None,
            validator_public_key: None,
            signature: None,
        };

        b.hash(&hasher)
    }

    // Create a new block based on the previous block header
    pub fn from_prev_header(
        prev_header: &BlockHeader,
        transactions: Vec<Transaction>,
        encoder: &DynEncoder,
        hasher: &DynHasher<Self>,
    ) -> Result<Self> {
        let header = BlockHeader {
            version: prev_header.version,
            height: prev_header.height + 1,
            timestamp: Instant::now().elapsed().as_nanos(),
            data_hash: data_hash(&transactions, encoder)?,
            prev_block_header_hash: Some(Block::hash_from_header(prev_header, hasher)?),
        };

        Block::new(header, transactions, encoder)
    }

    pub fn hash(&mut self, hasher: &DynHasher<Self>) -> Result<MyHash> {
        if let Some(hash) = self.hash {
            Ok(hash)
        } else {
            let hash = hasher.hash(self)?;
            self.hash = Some(hash);
            Ok(hash)
        }
    }

    pub fn sign(&mut self, private_key: &PrivateKey, enc: &DynEncoder) -> Result<()> {
        // Get header as bytes
        let bytes = self.header.bytes(enc)?;
        // Sign header bytes
        let signature = private_key.sign(bytes.as_slice());

        // Set the signature and public key
        self.validator_public_key = Some(private_key.public_key());
        self.signature = Some(signature);

        Ok(())
    }

    pub fn verify(&self, enc: &DynEncoder) -> Result<()> {
        // Check if the block has a signature
        let sig = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow!("block has no signature"))?;

        // Check if the block has a public key
        let pub_key = self
            .validator_public_key
            .as_ref()
            .ok_or_else(|| anyhow!("block has no validator (public_key)"))?;

        // Verify the signature
        if !sig.verify(&self.header.bytes(enc)?, pub_key) {
            return Err(anyhow!("block has invalid signature"));
        }

        // Verify every transactions
        // TODO: could probably make this faster by using a thread pool
        for tx in &self.transactions {
            tx.verify()?;
        }

        // Verify the data hash
        let data_hash = data_hash(&self.transactions, enc)?;

        if data_hash != self.header.data_hash {
            return Err(anyhow!("block has invalid data hash {}", data_hash));
        }

        Ok(())
    }

    pub fn encode(&self, encoder: &DynEncoder) -> Result<Vec<u8>> {
        encoder.encode(self)
    }
}

// hash all the transactions in the block
pub fn data_hash(transactions: &[Transaction], encoder: &DynEncoder) -> Result<MyHash> {
    let mut buf: Vec<u8> = vec![];
    for tx in transactions.iter() {
        let data = tx.encode(encoder)?;
        buf.extend_from_slice(&data);
    }
    let hash = Sha256::digest(buf.as_slice());
    Ok(MyHash::from_bytes(hash.as_slice()))
}

// TODO: find a way to include a secret message in the block
pub fn create_genesis_block(enc: &DynEncoder) -> Result<Block> {
    Block::new(
        BlockHeader {
            version: 1,
            height: 0,
            timestamp: tokio::time::Instant::now().elapsed().as_nanos(),
            prev_block_header_hash: None,
            data_hash: MyHash::zero(),
        },
        vec![],
        enc,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{hasher::BlockHasher, JsonEncoder},
        util::random_block,
    };

    use anyhow::Result;

    fn encoder() -> DynEncoder {
        Box::new(JsonEncoder {})
    }

    fn block_hasher() -> DynHasher<Block> {
        Box::new(BlockHasher::new(encoder()))
    }

    #[test]
    fn test_hash_block() -> Result<()> {
        let mut block = random_block(0, MyHash::zero(), &encoder())?;
        let hash = block.hash(&block_hasher())?;
        println!("hash: {hash}");
        Ok(())
    }

    #[test]
    fn test_sign_block() -> Result<()> {
        let private_key = PrivateKey::generate();
        let mut b = random_block(0, MyHash::zero(), &encoder())?;
        b.sign(&private_key, &encoder())?;
        assert!(b.signature.is_some());

        Ok(())
    }

    #[test]
    fn test_verify_block() -> Result<()> {
        let enc = encoder();
        let private_key = PrivateKey::generate();
        let mut b = random_block(0, MyHash::zero(), &enc)?;
        b.sign(&private_key, &enc)?;
        b.verify(&enc)?;

        // changing the data should make the public key invalid
        b.header.height = 100;
        assert!(b.verify(&enc).is_err());
        b.header.height = 0;

        // changing the public key should make the signature invalid
        let other_private_key = PrivateKey::generate();
        b.validator_public_key = Some(other_private_key.public_key());
        assert!(b.verify(&enc).is_err());

        Ok(())
    }
}
