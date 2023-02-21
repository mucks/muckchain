use crate::{
    core::{Block, Blockchain, DynDecoder, DynEncoder},
    crypto::PrivateKey,
};

use super::{node::EncodingConfig, DynTransport, TxPool};
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
}

impl Validator {
    pub fn new(
        config: ValidatorConfig,
        blockchain: Blockchain,
        tx_pool: TxPool,
        transport: DynTransport,
    ) -> Self {
        Self {
            config,
            blockchain,
            tx_pool,
            transport,
        }
    }
    // Start validator loop in another thread
    // clone self and move it to the new thread
    pub fn start_thread(&self) {
        let s = self.clone();

        tokio::spawn(async move {
            s.start().await;
        });
    }

    pub async fn start(&self) {
        loop {
            sleep(Duration::from_millis(self.config.block_time_ms)).await;
        }
    }

    fn broadcast_block(&self, block: Block) {
        let v = self.clone();
        tokio::spawn(async move {
            let data = match v.config.encoding.encoder.encode(&block) {
                Ok(data) => data,
                Err(err) => {
                    error!("Error encoding block: {}", err);
                    return;
                }
            };

            if let Err(err) = v.transport.broadcast(data).await {
                error!("Error broadcasting block: {}", err);
            }
        });
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

        self.broadcast_block(block);

        // broadcast block

        Ok(())
    }
}
