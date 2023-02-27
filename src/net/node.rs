use std::time::Duration;

use crate::{
    core::{
        create_genesis_block, Block, BlockHasher, BlockHeader, BlockValidator, BlockchainConfig,
        DefaultBlockValidator, DynDecoder, DynEncoder, DynHasher, Hasher, MemStorage, TxHasher,
    },
    model::MyHash,
};

use log::{debug, error, info};

use super::{
    message_sender::MessageSender,
    net_addr::NetAddr,
    rpc::{new_channel, Channel},
    transport::DynTransport,
    tx_pool,
    validator::{Validator, ValidatorConfig},
    LocalTransport, Network, TxPool,
};

use crate::{
    core::{Blockchain, JsonDecoder, JsonEncoder, Transaction},
    crypto::PrivateKey,
    net::message::Message,
    Result,
};

pub type NodeID = String;

#[derive(Debug, Clone)]
pub struct HasherConfig {
    pub tx_hasher: DynHasher<Transaction>,
    pub block_hasher: DynHasher<Block>,
}

#[derive(Debug, Clone)]
pub struct EncodingConfig {
    pub encoder: DynEncoder,
    pub decoder: DynDecoder,
}

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub encoding: EncodingConfig,
    pub hashers: HasherConfig,
}

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
}

impl Node {
    pub fn new(
        id: String,
        transport: DynTransport,
        config: NodeConfig,
        blockchain_config: BlockchainConfig,
        validator_config: Option<ValidatorConfig>,
    ) -> Self {
        let msg_sender = MessageSender::new(transport.clone(), config.encoding.encoder.clone());

        let mut node = Self {
            id,
            transport,
            rpc_channel: new_channel(),
            tx_pool: TxPool::new(),
            validator: None,
            blockchain: Blockchain::new(blockchain_config),
            config,
            msg_sender: msg_sender.clone(),
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
        node
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

    pub async fn start(&mut self) -> Result<()> {
        if let Some(validator) = &self.validator {
            validator.start_thread();
        }

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
                let msg = match Message::from_rpc(self.config.encoding.decoder.clone(), &rpc) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Error decoding message: {:?}", e);
                        continue;
                    }
                };

                self.process_message(msg).await;
            }
        }
    }

    async fn process_message(&self, msg: Message) {
        match msg {
            Message::Transaction(tx) => {
                if let Err(err) = self.process_transaction(tx).await {
                    error!("Error processing transaction: {:?}", err);
                };
            }
            Message::Text(text) => {
                debug!("Node={} received text={}", self.id, text);
            }
            Message::Block(block) => {
                if let Err(err) = self.process_block(block).await {
                    error!("Error processing block: {:?}", err);
                };
            }
        }
    }

    async fn process_block(&self, mut block: Block) -> Result<()> {
        let block_hash = block.hash(&self.config.hashers.block_hasher.clone())?;

        info!("Node={} received block={}", self.id, block_hash);

        self.blockchain.add_block(block).await?;

        // Check if we already have this block in our chain
        Ok(())
    }

    async fn process_transaction(&self, mut tx: Transaction) -> Result<()> {
        let tx_hash = tx.hash(self.config.hashers.tx_hasher.clone()).await?;

        info!("Node={} received transaction={}", self.id, tx_hash);

        // Check if we already have this transaction in our pool
        if self.tx_pool.has_tx(&tx_hash).await {
            debug!("Node={} already has transaction={}", self.id, tx_hash);
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
            .add_tx(self.config.hashers.tx_hasher.clone(), tx)
            .await
        {
            error!("could not add transaction to tx_pool: {:?}", err);
        }

        Ok(())
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

    let encoding_config = EncodingConfig {
        encoder: Box::new(JsonEncoder),
        decoder: Box::new(JsonDecoder),
    };

    let hasher_config = HasherConfig {
        tx_hasher: Box::new(TxHasher),
        block_hasher: Box::new(BlockHasher::new(encoding_config.encoder.clone())),
    };

    let config = NodeConfig {
        encoding: encoding_config.clone(),
        hashers: hasher_config.clone(),
    };

    // create blockchain config
    let blockchain_config = BlockchainConfig {
        genesis_block: create_genesis_block(&encoding_config.encoder)?,
        storage: Box::new(MemStorage {}),
        block_validator: Box::new(DefaultBlockValidator {}),
        encoding: encoding_config.clone(),
        hashers: hasher_config.clone(),
    };

    /*
        If the node is a validator we create a validator config which
        contains the private key of the validator
    */
    let mut validator_config = None;
    if let Some(private_key) = private_key {
        validator_config = Some(ValidatorConfig::new(
            encoding_config,
            hasher_config,
            private_key,
            5000,
        ));
    }

    /*
        Now we create a new Node with the transport so that we can
        send and broadcast messages to all nodes within this node
    */

    let node = Node::new(
        node_id.into(),
        Box::new(tr.clone()),
        config,
        blockchain_config,
        validator_config,
    );

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
