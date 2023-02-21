use super::{
    net_addr::NetAddr,
    node::NodeID,
    rpc::{new_channel, Channel, RPC},
    transport::{DynTransport, Transport},
};
use anyhow::Result;
use log::{debug, error};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

// Handles all the transports
#[derive(Debug, Clone)]
pub struct Network {
    transports: Arc<Mutex<HashMap<NetAddr, DynTransport>>>,
    rpc_channel: Channel,
    node_channels: Arc<Mutex<HashMap<NetAddr, Channel>>>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            transports: Arc::new(Mutex::new(HashMap::new())),
            rpc_channel: new_channel(),
            node_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn rpc_recv(&self) -> Option<RPC> {
        self.rpc_channel.1.lock().await.recv().await
    }

    // LOOP
    pub async fn listen(&self) {
        let rpc_recv = self.rpc_channel.1.clone();

        loop {
            if let Some(rpc) = rpc_recv.lock().await.recv().await {
                debug!("Network received message from {}", rpc.from);
            }
        }
    }

    pub async fn add_node_channel(
        &self,
        id: NodeID,
        transport_addr: NetAddr,
        node_channel: Channel,
    ) {
        debug!("Adding NodeChannel from Node={id} with Transport={transport_addr} to Network");
        self.node_channels
            .lock()
            .await
            .insert(transport_addr, node_channel);
    }

    pub async fn add_transport(&self, tr: DynTransport) -> Result<()> {
        debug!("Adding Transport={} to Network", tr.addr());

        self.transports.lock().await.insert(tr.addr(), tr.clone());
        self.connect_transport_to_transports(tr.clone()).await?;
        self.connect_transport_to_network(tr)?;
        Ok(())
    }

    // Connect all Transports with each other so that each Transport can send and broadcast data to one another
    async fn connect_transport_to_transports(&self, new_tr: DynTransport) -> Result<()> {
        for (tr_addr, tr) in self.transports.lock().await.clone() {
            if new_tr.addr() != tr_addr {
                new_tr.connect(tr.clone()).await?;
                tr.connect(new_tr.clone()).await?;
            }
        }

        Ok(())
    }

    // Connect the transport to the network and forward all messages to the network listen function
    fn connect_transport_to_network(&self, tr: DynTransport) -> Result<()> {
        let sender = self.rpc_channel.0.clone();
        let node_channels = self.node_channels.clone();

        tokio::spawn(async move {
            loop {
                if let Some(rpc) = tr.recv().await {
                    // Forward all transport to All added node channels
                    for (transport_id, node_channel) in node_channels.lock().await.clone() {
                        // Don't forward messages that are send to itself
                        if transport_id == rpc.from {
                            continue;
                        }

                        if let Err(err) = node_channel.0.send(rpc.clone()).await {
                            error!(
                                "Transport={} could not send RPC to Node with Transport={}, err: {}",
                                tr.addr(),
                                transport_id,
                                err
                            );
                        }
                    }

                    // Forward all transports to Network for debugging
                    debug!("Transport={} sending RPC to Network", tr.addr());
                    if let Err(err) = sender.send(rpc).await {
                        error!(
                            "Transport={} could not send RPC to Network, err: {}",
                            tr.addr(),
                            err
                        );
                    }
                }
            }
        });
        Ok(())
    }
}
