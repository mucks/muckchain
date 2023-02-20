/*
    The TxPool struct is used to store all the transactions that are not yet included in a block.
    The TxPool will store transactions that are not yet included in a block.
    It's supposed to be attached to a Node struct.
    Which means that it needs to be usable in many threads and therefore needs a shared state.

*/

use crate::core::Transaction;
use crate::model::MyHash;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct TxPool {
    transactions: Arc<RwLock<HashMap<MyHash, Transaction>>>,
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_transaction(&self, transaction: Transaction) {
        // self.transactions
        //     .write()
        //     .await
        //     .insert(transaction.hash, transaction);
    }

    pub async fn remove_transaction(&self, transaction: &Transaction) {
        // let mut transactions = self.transactions.write().await;
        // transactions.retain(|t| t != transaction);
    }

    pub async fn get_transactions(&self) -> Vec<Transaction> {
        todo!()
        // self.transactions.read().await.clone()
    }
}
