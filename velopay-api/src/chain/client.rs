use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};

pub type VelocityClient = OnlineClient<PolkadotConfig>;

pub async fn connect_to_chain(rpc_url: &str) -> Result<VelocityClient> {
    let client = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;
    Ok(client)
}

// TODO: Generate metadata.scale from velo-chain runtime before uncommenting
// Run: subxt metadata -f bytes > velopay-api/metadata.scale
// from a running velo-chain node
/*
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod velocity {}

pub use velocity::runtime_types;
*/
