mod net;
use anyhow::Result;
use net::{create_and_start_node, LocalTransport, Network, Node, NodeID, Transport};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let network = Network::new();

    let local = create_and_start_node(network.clone(), "LOCAL_NODE", "TR_LOCAL").await?;
    let remote = create_and_start_node(network.clone(), "REMOTE_NODE", "TR_REMOTE").await?;
    add_late_node(network.clone()).await?;

    network.listen().await;

    Ok(())
}

async fn add_late_node(network: Network) -> Result<()> {
    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        let late = create_and_start_node(network, "LATE_NODE", "TR_LATE")
            .await
            .unwrap();
    });
    Ok(())
}
