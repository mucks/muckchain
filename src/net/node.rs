use log::debug;

use super::{
    net_addr::NetAddr,
    rpc::{new_channel, Channel},
    transport::DynTransport,
    LocalTransport, Network,
};
use crate::Result;

pub type NodeID = String;

#[derive(Debug, Clone)]
pub struct Node {
    id: NodeID,
    transport: DynTransport,
    rpc_channel: Channel,
}

impl Node {
    pub fn new(id: String, transport: DynTransport) -> Self {
        Self {
            id,
            transport,
            rpc_channel: new_channel(),
        }
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

    pub async fn start(&self) -> Result<()> {
        self.transport
            .broadcast(format!("Starting Node={}", self.id).as_bytes().to_vec())
            .await?;

        self.listen().await;
        Ok(())
    }

    pub async fn listen(&self) {
        loop {
            if let Some(rpc) = self.rpc_channel.1.lock().await.recv().await {
                debug!("Node={} received RPC from={}", self.id, rpc.from);
            }
        }
    }
}

// TODO: maybe move this somewhere else as it explains the code quite well

pub async fn create_and_start_node(
    network: Network,
    node_id: &str,
    transport_addr: &str,
) -> Result<Node> {
    // First we create a transport which handles the sending of messages
    let tr = LocalTransport::new(transport_addr.into());
    /*
        Then we add that transport to the network where it get's forwarded to
        all the nodes on the network and to the network itself for debugging
    */
    network.add_transport(Box::new(tr.clone())).await?;

    /*
        Now we create a new Node with the transport so that we can
        send and broadcast messages to all nodes within this node
    */
    let node = Node::new(node_id.into(), Box::new(tr.clone()));

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
    let node_clone = node.clone();
    tokio::spawn(async move { node_clone.start().await });

    Ok(node)
}
