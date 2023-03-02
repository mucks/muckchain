use std::ops::Range;

use super::{DynTransport, Message, Status};
use crate::prelude::*;

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
    pub fn send_status_threaded(&self, to: NetAddr, status: Status) {
        let msg = Message::Status(status);
        self.send_threaded(to, msg);
    }

    pub fn send_get_blocks_threaded(&self, to: NetAddr, range: Range<u32>) {
        let msg = Message::GetBlocks(range);
        self.send_threaded(to, msg);
    }

    pub fn send_blocks_threaded(&self, to: NetAddr, blocks: Vec<Block>) {
        let msg = Message::Blocks(blocks);
        self.send_threaded(to, msg);
    }

    pub fn broadcast_get_blockchain_status_threaded(&self) {
        let msg = Message::GetStatus;
        self.broadcast_threaded(msg);
    }

    pub fn broadcast_transaction_threaded(&self, transaction: Transaction) {
        let msg = Message::Transaction(transaction);
        self.broadcast_threaded(msg);
    }

    pub fn broadcast_block_threaded(&self, block: Block) {
        let msg = Message::Block(block);
        self.broadcast_threaded(msg);
    }

    // Core function to send a message to a node
    async fn send(&self, to: &NetAddr, msg: Message) -> Result<()> {
        self.transport.send(to, msg.bytes(&self.encoder)?).await?;
        Ok(())
    }

    fn send_threaded(&self, to: NetAddr, msg: Message) {
        let s = self.clone();
        tokio::spawn(async move {
            if let Err(err) = s.send(&to, msg).await {
                error!("Error sending msg: {:?}", err);
            }
        });
    }

    // Core function to broadcast a message to all nodes in the network
    async fn broadcast(&self, msg: Message) -> Result<()> {
        self.transport.broadcast(msg.bytes(&self.encoder)?).await
    }

    fn broadcast_threaded(&self, msg: Message) {
        let s = self.clone();
        tokio::spawn(async move {
            if let Err(err) = s.broadcast(msg).await {
                error!("Error broadcasting msg: {:?}", err);
            }
        });
    }
}
