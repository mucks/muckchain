use crate::{
    core::{Block, Blockchain, DynDecoder, DynEncoder, Transaction},
    crypto::PrivateKey,
};

use super::{message_sender::MessageSender, node::EncodingConfig, DynTransport, TxPool};
use anyhow::{anyhow, Result};
use log::error;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    private_key: PrivateKey,
    block_time_ms: u64,
    encoding: EncodingConfig,
}

impl ValidatorConfig {
    pub fn new(encoding: EncodingConfig, private_key: PrivateKey, block_time_ms: u64) -> Self {
        Self {
            private_key,
            block_time_ms,
            encoding,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Validator {
    config: ValidatorConfig,
    blockchain: Blockchain,
    tx_pool: TxPool,
    transport: DynTransport,
    msg_sender: MessageSender,
}

impl Validator {
    pub fn new(
        config: ValidatorConfig,
        blockchain: Blockchain,
        tx_pool: TxPool,
        transport: DynTransport,
        msg_sender: MessageSender,
    ) -> Self {
        Self {
            config,
            blockchain,
            tx_pool,
            transport,
            msg_sender,
        }
    }

    // TODO: move this to a test
    fn send_signed_test_transaction(&self) {
        let mut tx = Transaction::new("hello world!".into());
        tx.sign(&self.config.private_key);

        let msg_sender = self.msg_sender.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            msg_sender.broadcast_transaction_thread(tx);
        });
    }

    // Start validator loop in another thread
    // clone self and move it to the new thread
    pub fn start_thread(&self) {
        // Send test transaction
        self.send_signed_test_transaction();

        let s = self.clone();

        tokio::spawn(async move {
            s.start().await;
        });
    }

    pub async fn start(&self) {
        loop {
            sleep(Duration::from_millis(self.config.block_time_ms)).await;
            if let Err(err) = self.create_new_block().await {
                error!("Error creating new block: {:?}", err);
            }
        }
    }

    // Create a new block and broadcast it to all the nodes in the network
    async fn create_new_block(&self) -> Result<()> {
        let bc_height = self.blockchain.height().await;
        let prev_header = self
            .blockchain
            .get_prev_header(bc_height)
            .await
            .ok_or_else(|| anyhow!("No previous block header found"))?;

        let pending_txs = self.tx_pool.pending().await?;

        let block = Block::new(prev_header, pending_txs);

        self.blockchain.add_block(block.clone()).await;

        self.tx_pool.clear_pending().await;

        self.msg_sender.broadcast_block_thread(block);

        Ok(())
    }
}
