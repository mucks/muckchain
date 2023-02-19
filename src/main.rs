use std::{fmt::format, sync::Arc, time::Duration};

use anyhow::Result;

use futures::{future::BoxFuture, FutureExt};
use log::{debug, error, info};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

// Remote Procedure call, used to communicatie through the network
#[derive(Debug, Default, Clone)]
pub struct RPC {
    pub from: String,
    pub data: Vec<u8>,
}

pub type Sndr = Sender<RPC>;
pub type Rcvr = Receiver<RPC>;
pub type AMRcvr = Arc<Mutex<Rcvr>>;
pub type Channel = (Sndr, AMRcvr);

#[derive(Debug, Clone)]
pub struct NodePool {
    nodes: Arc<Mutex<Vec<Node>>>,
    channel: Channel,
}

impl NodePool {
    pub fn new() -> Self {
        let c = tokio::sync::mpsc::channel(100);

        Self {
            nodes: Arc::new(Mutex::new(vec![])),
            channel: (c.0, Arc::new(Mutex::new(c.1))),
        }
    }

    // 1 recv loop that receives all messages from all nodes

    async fn recv_loop(&self) -> Result<()> {
        let rcvr = self.channel.1.clone();

        loop {
            if let Some(rpc) = rcvr.lock().await.recv().await {
                info!("NodePool Received Message from {}", rpc.from);
            }
        }
    }

    pub async fn wait(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    /*
        <Adding Node Functions>
    */

    // adds a node to the nodepool and
    pub async fn add_node(&self, node: Node) -> Result<()> {
        debug!("Adding {} to NodePool ...", node.id);
        self.nodes.lock().await.push(node.clone());
        self.connect_node_to_recv_loop(node);
        Ok(())
    }

    // connects the node to the NodePools receive pool
    // when this node receives a message it will be forwarded to the NodePool
    // in the function recv_loop
    pub fn connect_node_to_recv_loop(&self, node: Node) {
        let sender = self.channel.0.clone();

        tokio::spawn(async move {
            loop {
                if let Some(rpc) = node.channel.1.lock().await.recv().await {
                    debug!("{} sending rpc to NodePool", node.id);
                    if let Err(err) = sender.send(rpc).await {
                        error!("could not send to NodePool, err: {}", err);
                    }
                }
            }
        });
    }

    /*
        </Adding Node Functions>
    */

    // Broadcast should send one message to all nodes
    pub async fn broadcast(&self, sender_id: String, rpc: RPC) -> Result<()> {
        // only if sender == node
        for node in self.nodes().await {
            let rpc = rpc.clone();
            let sender_id = sender_id.clone();
            tokio::spawn(async move {
                if node.id == sender_id {
                    if let Err(err) = node.channel.0.send(rpc.clone()).await {
                        error!("error sending, err: {}", err);
                    }
                }
            });
        }
        Ok(())
    }

    pub async fn nodes(&self) -> Vec<Node> {
        self.nodes.lock().await.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub channel: Channel,
    // pub peers: Vec<Channel>,
    // pub nodes: &'a Vec<Node<'a>>,
}

impl Node {
    pub fn new(id: String) -> Self {
        let c = tokio::sync::mpsc::channel(100);
        Self {
            id,
            channel: (c.0, Arc::new(Mutex::new(c.1))),
            // channel: tokio::sync::mpsc::channel(100),
            // peers: vec![],
        }
    }

    pub async fn start(&self) {
        loop {
            if let Err(err) = self
                .channel
                .0
                .send(RPC {
                    from: self.id.clone(),
                    data: b"hello world".to_vec(),
                })
                .await
            {
                error!("can't send message to recv loop, err: {}", err);
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let node_pool = NodePool::new();

    let node_a = Node::new("NODE_A".to_string());
    node_pool.add_node(node_a).await?;

    let node_b = Node::new("NODE_B".to_string());
    node_pool.add_node(node_b).await?;

    add_late_node(node_pool.clone());

    for node in node_pool.nodes().await {
        tokio::spawn(async move {
            node.start().await;
        });
    }

    node_pool.recv_loop().await?;

    Ok(())
}

fn add_late_node(node_pool: NodePool) {
    tokio::spawn(async move {
        println!("Waiting 2 seconds...");
        tokio::time::sleep(Duration::from_secs(2)).await;

        let node_c = Node::new("NODE_C_DELAYED".to_string());
        node_pool.add_node(node_c.clone()).await.unwrap();
        node_c.start().await;
    });
}
