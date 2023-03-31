use crate::prelude::Encodable;

use super::net_addr::NetAddr;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Sender = tokio::sync::mpsc::Sender<RPC>;
pub type Receiver = tokio::sync::mpsc::Receiver<RPC>;
pub type Channel = (Sender, Arc<Mutex<Receiver>>);

pub fn new_channel() -> Channel {
    let c = tokio::sync::mpsc::channel(1024);
    (c.0, Arc::new(Mutex::new(c.1)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPC {
    pub from: NetAddr,
    pub data: Vec<u8>,
}

#[typetag::serde]
impl Encodable for RPC {}
