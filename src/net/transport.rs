use super::{
    net_addr::NetAddr,
    rpc::{Sender, RPC},
};
use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;

pub type DynTransport = Box<dyn Transport>;

#[async_trait]
pub trait Transport: TransportClone + Send + Sync + Debug {
    async fn broadcast(&self, data: Vec<u8>) -> Result<()>;
    async fn send(&self, to: &NetAddr, data: Vec<u8>) -> Result<()>;
    async fn connect(&self, tr: Box<dyn Transport>) -> Result<()>;
    fn sender(&self) -> Sender;
    fn addr(&self) -> NetAddr;
    async fn recv(&self) -> Option<RPC>;
}

pub trait TransportClone {
    fn clone_box(&self) -> Box<dyn Transport>;
}

impl<T> TransportClone for T
where
    T: 'static + Transport + Clone,
{
    fn clone_box(&self) -> Box<dyn Transport> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Transport> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
