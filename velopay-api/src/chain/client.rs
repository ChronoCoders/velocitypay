use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};

pub type VelocityClient = OnlineClient<PolkadotConfig>;

pub async fn connect_to_chain(rpc_url: &str) -> Result<VelocityClient> {
    let client = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;
    Ok(client)
}

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod velocity {}

pub use velocity::runtime_types;
