use std::{collections::HashMap, sync::Arc};

use crate::{
    net::rpc::{self, Sender, RPC},
    prelude::NetAddr,
};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::debug;
use tokio::sync::{Mutex, RwLock};

use super::{DynTransport, Transport};

#[derive(Debug, Clone)]
pub struct LocalTransport {
    addr: NetAddr,
    channel: rpc::Channel,
    // TODO: figure out if RwLock or Mutex is better here
    peers: Arc<RwLock<HashMap<NetAddr, DynTransport>>>,
}

impl LocalTransport {
    pub fn new(addr: NetAddr) -> Self {
        let c = tokio::sync::mpsc::channel(100);
        Self {
            addr,
            channel: (c.0, Arc::new(Mutex::new(c.1))),
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn listen(&self) {
        let c = self.channel.clone();
        let peers = self.peers.clone();
        let addr = self.addr.clone();

        tokio::spawn(async move {
            loop {
                if let Some(rpc) = c.1.lock().await.recv().await {
                    debug!("LocalTransport={} received message from {}", addr, rpc.from);

                    let mut peers_mut = peers.write().await;
                    let peer = peers_mut
                        .get_mut(&rpc.from)
                        .ok_or_else(|| {
                            anyhow!("LocalTransport={} could not find peer={}", addr, rpc.from)
                        })
                        .unwrap();

                    peer.sender().send(rpc).await.unwrap();
                }
            }
        });
    }
}

#[async_trait]
impl Transport for LocalTransport {
    async fn broadcast(&self, data: Vec<u8>) -> Result<()> {
        for (addr, _peer) in self.peers.read().await.clone() {
            self.send(&addr, data.clone()).await?
        }
        Ok(())
    }

    fn sender(&self) -> Sender {
        self.channel.0.clone()
    }

    async fn send(&self, to: &NetAddr, data: Vec<u8>) -> Result<()> {
        let peers = self.peers.read().await;
        let peer = peers
            .get(to)
            .ok_or_else(|| anyhow!("LocalTransport={} could not find peer={}", self.addr, to))?;

        let r = RPC {
            from: self.addr(),
            data,
        };

        peer.sender().send(r).await?;

        Ok(())
    }

    async fn connect(&self, tr: Box<dyn Transport>) -> Result<()> {
        let tr_addr = tr.addr();

        debug!(
            "Connecting LocalTransport={} to Transport={}",
            self.addr, tr_addr
        );

        // LocalTransport can't connect to itself
        if self.addr == tr.addr() {
            return Err(anyhow!("LocalTransport={} can't add Transport={} as peer because it has the same address as itself", self.addr, tr_addr));
        }

        let mut peers_mut = self.peers.write().await;

        // LocalTransport should not add same connection twice
        if peers_mut.contains_key(&tr_addr) {
            return Err(anyhow!(
                "LocalTransport={} already contains {}",
                self.addr,
                tr_addr
            ));
        }

        peers_mut.insert(tr_addr.clone(), tr);

        Ok(())
    }

    fn addr(&self) -> NetAddr {
        self.addr.clone()
    }

    async fn recv(&self) -> Option<rpc::RPC> {
        self.channel.1.lock().await.recv().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_connect() -> Result<()> {
        let a = LocalTransport::new("A".into());
        let b = LocalTransport::new("B".into());

        a.connect(Box::new(b.clone())).await?;
        b.connect(Box::new(a.clone())).await?;

        Ok(())
    }

    #[tokio::test]
    pub async fn test_send_message() -> Result<()> {
        let a = LocalTransport::new("A".into());
        let b = LocalTransport::new("B".into());

        a.connect(Box::new(b.clone())).await?;
        b.connect(Box::new(a.clone())).await?;

        let data = b"hello world".to_vec();
        a.send(&b.addr, data.clone()).await?;

        let rpc = b
            .recv()
            .await
            .ok_or_else(|| anyhow!("received rpc is none"))?;

        assert_eq!(data, rpc.data);

        Ok(())
    }
}
