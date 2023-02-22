use log::error;

use super::{message::Message, DynTransport};
use crate::core::{Block, DynEncoder, Transaction};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct MessageSender {
    transport: DynTransport,
    encoder: DynEncoder,
}

impl MessageSender {
    pub fn new(transport: DynTransport, encoder: DynEncoder) -> Self {
        Self { transport, encoder }
    }
}

impl MessageSender {
    pub fn broadcast_transaction_thread(&self, transaction: Transaction) {
        let s = self.clone();

        tokio::spawn(async move {
            if let Err(err) = s.broadcast_transaction(transaction).await {
                error!("Error broadcasting transaction: {:?}", err);
            }
        });
    }

    // Send a transaction to all nodes in the network
    pub async fn broadcast_transaction(&self, transaction: Transaction) -> Result<()> {
        let msg = Message::Transaction(transaction);
        self.transport.broadcast(msg.bytes(&self.encoder)?).await?;

        Ok(())
    }

    pub fn broadcast_block_thread(&self, block: Block) {
        let s = self.clone();
        tokio::spawn(async move {
            if let Err(err) = s.broadcast_block(block).await {
                error!("Error broadcasting block: {:?}", err);
            }
        });
    }

    pub async fn broadcast_block(&self, block: Block) -> Result<()> {
        let msg = Message::Block(block);
        self.transport.broadcast(msg.bytes(&self.encoder)?).await
    }
}
