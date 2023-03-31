use super::{
    net_addr::NetAddr,
    rpc::{Sender, RPC},
};
use anyhow::Result;
use async_trait::async_trait;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub type DynTransport = Box<dyn Transport>;

mod local_transport;
pub use local_transport::LocalTransport;
mod tcp_transport;
pub use tcp_transport::TcpTransport;

#[async_trait]
pub trait Transport: Send + Sync + Debug + DynClone {
    async fn broadcast(&self, data: Vec<u8>) -> Result<()>;
    async fn send(&self, to: &NetAddr, data: Vec<u8>) -> Result<()>;
    async fn connect(&self, tr: Box<dyn Transport>) -> Result<()>;
    fn sender(&self) -> Sender;
    fn addr(&self) -> NetAddr;
    async fn recv(&self) -> Option<RPC>;
}

dyn_clone::clone_trait_object!(Transport);
