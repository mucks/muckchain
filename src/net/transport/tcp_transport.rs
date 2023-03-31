use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use crate::{
    net::rpc::{Sender, RPC},
    prelude::*,
};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpSocket, TcpStream},
    sync::{Mutex, RwLock},
};

use super::{DynTransport, Transport};

#[derive(Debug)]
pub struct TcpPeer {
    addr: NetAddr,
    stream: TcpStream,
    transport: DynTransport,
}

#[derive(Debug, Clone)]
pub struct TcpTransport {
    addr: NetAddr,
    // stream: TcpStream,
    listener: Arc<Mutex<TcpListener>>,
    peers: Arc<RwLock<HashMap<NetAddr, TcpPeer>>>,
    encoder: DynEncoder,
}

impl TcpTransport {
    pub async fn new(addr: NetAddr, encoder: DynEncoder) -> Result<Self> {
        let listener = TcpListener::bind(addr.parse::<SocketAddr>()?).await?;

        Ok(Self {
            addr,
            listener: Arc::new(Mutex::new(listener)),
            peers: Arc::new(RwLock::new(HashMap::new())),
            encoder,
        })
    }

    pub async fn listen(&self) {
        let peers = self.peers.clone();
        let addr = self.addr.clone();
        let listener = self.listener.clone();

        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.lock().await.accept().await.unwrap();
            }
        });
    }
}

#[async_trait]
impl Transport for TcpTransport {
    async fn broadcast(&self, data: Vec<u8>) -> Result<()> {
        let peers = self.peers.read().await;
        for peer in peers.values() {
            self.send(&peer.addr, data.clone()).await?;
        }
        Ok(())
    }

    async fn send(&self, to: &NetAddr, data: Vec<u8>) -> Result<()> {
        let mut peers = self.peers.write().await;
        let peer = peers
            .get_mut(to)
            .ok_or_else(|| anyhow!("could not find peer={}", to))?;

        let r = RPC {
            from: self.addr.clone(),
            data,
        };

        let encoded = self.encoder.encode(&r)?;

        peer.stream.write_all(&encoded).await?;

        // TODO: send data to peer
        //peer.send(&self.addr.clone(), data).await

        Ok(())
    }

    async fn connect(&self, tr: Box<dyn Transport>) -> Result<()> {
        let mut peers = self.peers.write().await;

        let tr_addr = tr.addr().parse::<SocketAddr>()?;

        let stream = TcpStream::connect(tr_addr).await.unwrap();

        let peer = TcpPeer {
            addr: tr_addr.to_string(),
            stream,
            transport: tr,
        };

        peers.insert(tr_addr.to_string(), peer);
        Ok(())
    }

    fn sender(&self) -> Sender {
        todo!()
    }

    fn addr(&self) -> NetAddr {
        self.addr.clone()
    }

    async fn recv(&self) -> Option<RPC> {
        todo!()
    }
}
