use super::{message::Message, message_sender::MessageSender, TxPool};
use crate::{config::HasherConfig, net::Status, prelude::*};

#[derive(Debug, Clone)]
pub struct MessageProcessor {
    blockchain: Blockchain,
    node_id: String,
    hashers: HasherConfig,
    tx_pool: TxPool,
    sender: MessageSender,
}

impl MessageProcessor {
    pub fn new(
        node_id: String,
        blockchain: Blockchain,
        hashers: HasherConfig,
        tx_pool: TxPool,
        sender: MessageSender,
    ) -> Self {
        Self {
            node_id,
            blockchain,
            hashers,
            tx_pool,
            sender,
        }
    }

    pub async fn process_message(&self, from: NetAddr, msg: Message) -> Result<()> {
        match msg {
            Message::Transaction(tx) => self.process_transaction(tx).await?,
            Message::Block(block) => self.process_block(block).await?,
            // TODO: this was added for debug purposes, maybe remove it
            Message::Text(text) => {
                debug!("Node={} received text={}", self.node_id, text);
            }
            // Get Status Request from a peer, send back our status
            Message::GetStatus => {
                debug!("Node={} received GetStatus", self.node_id);
                let height = self.blockchain.height().await;
                let status = Status {
                    id: self.node_id.clone(),
                    height,
                };
                self.sender.send_status_threaded(from, status);
            }
            Message::Status(status) => {
                debug!("Node={} received Status={:?}", self.node_id, status);
            }
        }
        Ok(())
    }

    pub async fn process_block(&self, mut block: Block) -> Result<()> {
        let block_hash = block.hash(&self.hashers.block_hasher.clone())?;

        info!("Node={} received block={}", self.node_id, block_hash);

        self.blockchain.add_block(block.clone()).await?;

        self.sender.broadcast_block_threaded(block);

        // Check if we already have this block in our chain
        Ok(())
    }

    pub async fn process_transaction(&self, mut tx: Transaction) -> Result<()> {
        let tx_hash = tx.hash(self.hashers.tx_hasher.clone()).await?;

        info!("Node={} received transaction={}", self.node_id, tx_hash);

        // Check if we already have this transaction in our pool
        if self.tx_pool.has_tx(&tx_hash).await {
            debug!("Node={} already has transaction={}", self.node_id, tx_hash);
            return Ok(());
        }
        // Set the date we first saw this transaction: used for sorting
        // TODO: figure out if theres a better way to do this since it requires the tx to be mut
        let first_seen = tokio::time::Instant::now().elapsed().as_nanos();
        tx.set_first_seen(first_seen);

        // Verify the transaction
        tx.verify()?;

        if let Err(err) = self
            .tx_pool
            .add_tx(self.hashers.tx_hasher.clone(), tx.clone())
            .await
        {
            error!("could not add transaction to tx_pool: {:?}", err);
        }

        self.sender.broadcast_transaction_threaded(tx);

        Ok(())
    }
}
