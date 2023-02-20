use rand::{thread_rng, Rng};

use crate::core::Transaction;

pub fn random_transaction() -> Transaction {
    Transaction {
        data: thread_rng().gen::<[u8; 32]>().to_vec(),
        hash: None,
    }
}