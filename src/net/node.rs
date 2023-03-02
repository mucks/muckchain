use crate::config::{Config, NodeConfig, ValidatorConfig};
use crate::core::{BlockchainConfig, McError};
use crate::crypto::PrivateKey;
use crate::net::message::Message;
use crate::prelude::*;

use super::validator::Validator;
use super::DynTransport;
use super::{
    message_processor::MessageProcessor,
    message_sender::MessageSender,
    rpc::{new_channel, Channel},
    tx_pool, LocalTransport, Network, TxPool,
};

pub type NodeID = String;

// Every value with state needs to be clonable in a way so that it can be moved to another thread
// and still be usable and mutable
#[derive(Debug, Clone)]
pub struct Node {
    id: NodeID,
    transport: DynTransport,
    rpc_channel: Channel,
    tx_pool: tx_pool::TxPool,
    config: NodeConfig,
    validator: Option<Validator>,
    blockchain: Blockchain,
    msg_sender: MessageSender,
    msg_processor: MessageProcessor,
}

impl Node {
    pub async fn new(
        id: String,
        transport: DynTransport,
        config: NodeConfig,
        blockchain_config: BlockchainConfig,
        validator_config: Option<ValidatorConfig>,
    ) -> Result<Self> {
        let blockchain = Blockchain::new(blockchain_config).await?;
        let tx_pool = TxPool::new();

        let msg_sender = MessageSender::new(transport.clone(), config.encoding.encoder.clone());
        let msg_processor = MessageProcessor::new(
            id.clone(),
            blockchain.clone(),
            config.hashers.clone(),
            tx_pool,
            msg_sender.clone(),
        );

        let mut node = Self {
            id,
            transport,
            rpc_channel: new_channel(),
            tx_pool: TxPool::new(),
            validator: None,
            blockchain,
            config,
            msg_sender: msg_sender.clone(),
            msg_processor,
        };

        if let Some(validator_config) = validator_config {
            debug!("Node {} is a validator", node.id);
            node.validator = Some(Validator::new(
                validator_config,
                node.blockchain.clone(),
                node.tx_pool.clone(),
                node.transport.clone(),
                msg_sender,
            ));
        }

        Ok(node)
    }

    pub fn channel(&self) -> Channel {
        self.rpc_channel.clone()
    }

    pub fn id(&self) -> NodeID {
        self.id.clone()
    }

    pub fn transport_addr(&self) -> NetAddr {
        self.transport.addr()
    }

    // START
    pub async fn start(&mut self) -> Result<()> {
        if let Some(validator) = &self.validator {
            validator.start_thread();
        }

        //Send a get_blockchain_status message to all nodes
        self.msg_sender.broadcast_get_blockchain_status_threaded();

        // let msg = Message::Text("hello".into());

        // self.transport
        //     .broadcast(self.config.encoding.encoder.encode(&msg)?)
        //     .await?;

        self.listen().await;
        Ok(())
    }

    // Listen to the rpc_channel for new messages
    pub async fn listen(&self) {
        loop {
            if let Some(rpc) = self.rpc_channel.1.lock().await.recv().await {
                debug!("Node={} received RPC from={}", self.id, rpc.from);

                // Decode the message
                let msg = match Message::from_rpc(&self.config.encoding.decoder, &rpc) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Error decoding message: {:?}", e);
                        continue;
                    }
                };

                if let Err(err) = self.msg_processor.process_message(rpc.from, msg).await {
                    if let Some(mc_err) = err.downcast_ref::<McError>() {
                        match mc_err {
                            // Don't print an error if the block already exists
                            McError::BlockAlreadyExists(_) => {}
                        }
                    } else {
                        error!("Node={} Error processing message: {:?}", self.id, err);
                    }
                }
            }
        }
    }
}

// TODO: maybe move this somewhere else as it explains the code quite well

pub async fn create_and_start_node(
    network: Network,
    node_id: &str,
    transport_addr: &str,
    private_key: Option<PrivateKey>,
) -> Result<Node> {
    // First we create a transport which handles the sending of messages
    let tr = LocalTransport::new(transport_addr.into());
    /*
        Then we add that transport to the network where it get's forwarded to
        all the nodes on the network and to the network itself for debugging
    */
    network.add_transport(Box::new(tr.clone())).await?;

    /*
        Now we create a config where we can define the dynamic traits
        that configure for instance which encoder to use for this node

    */

    let config = Config::default();

    /*
        If the node is a validator we create a validator config which
        contains the private key of the validator
    */
    let mut validator_config = None;
    if let Some(private_key) = private_key {
        validator_config = Some(config.validator_config(private_key));
    }

    /*
        Now we create a new Node with the transport so that we can
        send and broadcast messages to all nodes within this node
    */

    let node = Node::new(
        node_id.into(),
        Box::new(tr.clone()),
        config.node_config(),
        config.blockchain_config(),
        validator_config,
    )
    .await?;

    /*
        After the creation we add the nodes rpc_channel to the network
        in order to allow the node itself to be forwarded by messages from
        transport
    */
    network
        .add_node_channel(node.id(), node.transport_addr(), node.channel())
        .await;

    /*
        Finally we start a new Async Task in order to start the node and listen
        to those forwarded messages
    */
    let mut node_clone = node.clone();
    tokio::spawn(async move { node_clone.start().await });

    Ok(node)
}
