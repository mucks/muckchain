use crate::{
    core::{data_hash, TxHasher},
    crypto::PrivateKey,
    prelude::*,
};
use anyhow::Result;
use rand::{thread_rng, Rng};

pub fn random_hash() -> Hash {
    Hash::from_bytes(thread_rng().gen::<[u8; 32]>().as_slice())
}

pub fn random_transaction() -> Transaction {
    Transaction::new(thread_rng().gen::<[u8; 32]>().to_vec())
}

pub fn random_block(height: u32, prev_block_header_hash: Hash, enc: &DynEncoder) -> Result<Block> {
    let private_key = PrivateKey::generate();
    let mut tx = random_transaction();
    let _hash = tx.hash(Box::new(TxHasher {}));

    let header = BlockHeader {
        version: 1,
        height,
        timestamp: tokio::time::Instant::now().elapsed().as_nanos(),
        prev_block_header_hash: Some(prev_block_header_hash),
        data_hash: Hash::zero(),
    };

    let mut b = Block::new(header, vec![]);
    b.header.data_hash = data_hash(&b.transactions, enc)?;
    b.sign(&private_key, enc)?;
    Ok(b)
}

pub fn random_block_with_signature(
    height: u32,
    prev_block_header_hash: Hash,
    enc: &DynEncoder,
) -> Result<Block> {
    let private_key = PrivateKey::generate();
    let mut b = random_block(height, prev_block_header_hash, enc)?;
    b.sign(&private_key, enc)?;
    Ok(b)
}
