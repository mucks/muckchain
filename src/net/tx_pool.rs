/*
    The TxPool struct is used to store all the transactions that are not yet included in a block.
    The TxPool will store transactions that are not yet included in a block.
    It's supposed to be attached to a Node struct.
    Which means that it needs to be usable in many threads and therefore needs a shared state.

*/

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TxPool {
    all_txs: Arc<RwLock<HashMap<Hash, Transaction>>>,
    pending_txs: Arc<RwLock<HashMap<Hash, Transaction>>>,
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            all_txs: Arc::new(RwLock::new(HashMap::new())),
            pending_txs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_tx(
        &self,
        hasher: Box<dyn Hasher<Transaction>>,
        mut tx: Transaction,
    ) -> Result<()> {
        let tx_hash = tx.hash(hasher).await?;
        debug!("adding tx to pool: {:?}", tx_hash);
        self.all_txs.write().await.insert(tx_hash, tx.clone());
        self.pending_txs.write().await.insert(tx_hash, tx);

        Ok(())
    }
    pub async fn pending(&self) -> Result<Vec<Transaction>> {
        let mut txs: Vec<Transaction> = self.pending_txs.read().await.values().cloned().collect();

        // If some transactions have no valid timestamp return an error
        if txs.iter().any(|tx| tx.first_seen() == 0) {
            return Err(anyhow!("some transaction have first_seen set to 0 which means it didn't get set when it arrived at the node, make sure to set_first_seen after you receive a transaction"));
        }

        txs.sort_by_key(|a| a.first_seen());

        Ok(txs)
    }

    pub async fn has_tx(&self, tx_hash: &Hash) -> bool {
        self.all_txs.read().await.contains_key(tx_hash)
    }

    pub async fn clear_pending(&self) {
        self.pending_txs.write().await.clear();
    }

    pub async fn remove_tx(&self, tx: &Transaction) {
        // let mut transactions = self.transactions.write().await;
        // transactions.retain(|t| t != transaction);
    }

    pub async fn get_tx(&self) -> Vec<Transaction> {
        todo!()
        // self.transactions.read().await.clone()
    }
}
