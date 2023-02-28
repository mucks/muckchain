mod config;
mod core;
mod crypto;
mod net;
mod util;

use anyhow::Result;
use crypto::PrivateKey;
use net::{create_and_start_node, Network};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let network = Network::new();

    let private_key = PrivateKey::generate();
    let local =
        create_and_start_node(network.clone(), "LOCAL_NODE", "TR_LOCAL", Some(private_key)).await?;

    let remote = create_and_start_node(network.clone(), "REMOTE_NODE", "TR_REMOTE", None).await?;
    add_late_node(network.clone()).await?;

    network.listen().await;

    Ok(())
}

async fn add_late_node(network: Network) -> Result<()> {
    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        let late = create_and_start_node(network, "LATE_NODE", "TR_LATE", None)
            .await
            .unwrap();
    });
    Ok(())
}

pub mod prelude {
    pub use crate::core::{
        Block, BlockHeader, Blockchain, Decodable, DynDecoder, DynEncoder, DynHasher, Encodable,
        Hash, Hasher, Transaction,
    };
    pub use crate::net::{NetAddr, NodeID};
    pub use anyhow::{anyhow, Result};
    pub use async_trait::async_trait;
    pub use log::{debug, error, info, trace, warn};
    pub use serde::{Deserialize, Serialize};
    pub use tokio::time::{sleep, Duration, Instant};
}
