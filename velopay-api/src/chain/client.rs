use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};

pub type VelocityClient = OnlineClient<PolkadotConfig>;

pub async fn connect_to_chain(rpc_url: &str) -> Result<VelocityClient> {
    let client = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;
    Ok(client)
}

// Generate typed interfaces from velo-chain runtime metadata
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod velo_runtime {}

// Re-export runtime types for advanced usage
#[allow(unused_imports)]
pub use velo_runtime::runtime_types;
