use anyhow::Result;
use rand::{thread_rng, Rng};

use crate::{
    core::{data_hash, Block, BlockHeader, DynEncoder, Transaction, TxHasher},
    crypto::PrivateKey,
    model::MyHash,
};

pub fn random_hash() -> MyHash {
    MyHash::from_bytes(thread_rng().gen::<[u8; 32]>().as_slice())
}

pub fn random_transaction() -> Transaction {
    Transaction::new(thread_rng().gen::<[u8; 32]>().to_vec())
}

pub fn random_block(
    height: u32,
    prev_block_header_hash: MyHash,
    enc: &DynEncoder,
) -> Result<Block> {
    let private_key = PrivateKey::generate();
    let mut tx = random_transaction();
    let _hash = tx.hash(Box::new(TxHasher {}));

    let header = BlockHeader {
        version: 1,
        height,
        timestamp: tokio::time::Instant::now().elapsed().as_nanos(),
        prev_block_header_hash: Some(prev_block_header_hash),
        data_hash: MyHash::zero(),
    };

    Block::new(header, vec![], enc)
}

pub fn random_block_with_signature(
    height: u32,
    prev_block_header_hash: MyHash,
    enc: &DynEncoder,
) -> Result<Block> {
    let private_key = PrivateKey::generate();
    let mut b = random_block(height, prev_block_header_hash, enc)?;
    b.sign(&private_key, enc)?;
    Ok(b)
}
