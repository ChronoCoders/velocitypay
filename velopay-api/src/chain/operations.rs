use anyhow::{Result, anyhow};
use rust_decimal::Decimal;
use sp_core::{sr25519::Pair, Pair as PairT, crypto::Ss58Codec};
use sp_keyring::AccountKeyring;
use subxt::tx::PairSigner;
use tokio::time::{timeout, Duration};

use super::client::{VelocityClient, velo_runtime};

// Timeout for blockchain operations (30 seconds)
const BLOCKCHAIN_TIMEOUT_SECS: u64 = 30;

/// Blockchain operations service for submitting extrinsics
pub struct ChainOperations {
    signer: PairSigner<subxt::PolkadotConfig, Pair>,
}

impl ChainOperations {
    /// Create new blockchain operations service with admin signer
    pub fn new(admin_seed: &str) -> Result<Self> {
        // For development, support //Alice, //Bob, etc.
        let pair = if admin_seed.starts_with("//") {
            match admin_seed {
                "//Alice" => AccountKeyring::Alice.pair(),
                "//Bob" => AccountKeyring::Bob.pair(),
                "//Charlie" => AccountKeyring::Charlie.pair(),
                "//Dave" => AccountKeyring::Dave.pair(),
                "//Eve" => AccountKeyring::Eve.pair(),
                "//Ferdie" => AccountKeyring::Ferdie.pair(),
                _ => {
                    // Try parsing as a dev seed
                    Pair::from_string(admin_seed, None)
                        .map_err(|e| anyhow!("Failed to create keypair from seed '{}': {:?}", admin_seed, e))?
                }
            }
        } else {
            // Production seed phrase
            Pair::from_string(admin_seed, None)
                .map_err(|e| anyhow!("Failed to create keypair from seed: {:?}", e))?
        };

        let signer = PairSigner::new(pair);

        Ok(Self { signer })
    }

    /// Request a mint operation on the blockchain
    /// Note: The signer (admin) requests mint on behalf of the user
    pub async fn request_mint(
        &self,
        client: &VelocityClient,
        amount: Decimal,
    ) -> Result<(String, u64)> {
        // Convert Decimal to u128 (assuming 12 decimals for token)
        let amount_u128 = Self::decimal_to_balance(amount)?;

        // Build the extrinsic - request_mint only takes amount (signer is the beneficiary)
        let mint_call = velo_runtime::tx()
            .velo_pay()
            .request_mint(amount_u128);

        // Submit and watch the extrinsic with timeout
        let result = timeout(
            Duration::from_secs(BLOCKCHAIN_TIMEOUT_SECS),
            async {
                client
                    .tx()
                    .sign_and_submit_then_watch_default(&mint_call, &self.signer)
                    .await?
                    .wait_for_finalized_success()
                    .await
            }
        )
        .await
        .map_err(|_| anyhow!("Blockchain operation timed out after {} seconds", BLOCKCHAIN_TIMEOUT_SECS))??;

        // Get the transaction hash
        let tx_hash = format!("0x{}", hex::encode(result.extrinsic_hash()));

        // Extract request_id from events
        let request_id = Self::extract_mint_request_id(&result)?;

        log::info!("Mint request submitted - tx_hash: {}, request_id: {}", tx_hash, request_id);
        log::debug!("Mint request amount: {}", amount);

        Ok((tx_hash, request_id))
    }

    /// Approve a mint request on the blockchain
    pub async fn approve_mint(
        &self,
        client: &VelocityClient,
        request_id: u64,
    ) -> Result<String> {
        let approve_call = velo_runtime::tx()
            .velo_pay()
            .approve_mint(request_id);

        let result = timeout(
            Duration::from_secs(BLOCKCHAIN_TIMEOUT_SECS),
            async {
                client
                    .tx()
                    .sign_and_submit_then_watch_default(&approve_call, &self.signer)
                    .await?
                    .wait_for_finalized_success()
                    .await
            }
        )
        .await
        .map_err(|_| anyhow!("Blockchain operation timed out after {} seconds", BLOCKCHAIN_TIMEOUT_SECS))??;

        let tx_hash = format!("0x{}", hex::encode(result.extrinsic_hash()));

        log::info!("Mint approved - request_id: {}", request_id);
        log::debug!("Mint approval tx_hash: {}", tx_hash);

        Ok(tx_hash)
    }

    /// Reject a mint request on the blockchain
    pub async fn reject_mint(
        &self,
        client: &VelocityClient,
        request_id: u64,
    ) -> Result<String> {
        let reject_call = velo_runtime::tx()
            .velo_pay()
            .reject_mint(request_id);

        let result = timeout(
            Duration::from_secs(BLOCKCHAIN_TIMEOUT_SECS),
            async {
                client
                    .tx()
                    .sign_and_submit_then_watch_default(&reject_call, &self.signer)
                    .await?
                    .wait_for_finalized_success()
                    .await
            }
        )
        .await
        .map_err(|_| anyhow!("Blockchain operation timed out after {} seconds", BLOCKCHAIN_TIMEOUT_SECS))??;

        let tx_hash = format!("0x{}", hex::encode(result.extrinsic_hash()));

        log::info!("Mint rejected - request_id: {}", request_id);
        log::debug!("Mint rejection tx_hash: {}", tx_hash);

        Ok(tx_hash)
    }

    /// Request a burn operation on the blockchain
    /// Note: The signer (user) requests to burn their own tokens
    pub async fn request_burn(
        &self,
        client: &VelocityClient,
        amount: Decimal,
    ) -> Result<(String, u64)> {
        let amount_u128 = Self::decimal_to_balance(amount)?;

        // request_burn only takes amount (signer is the token holder)
        let burn_call = velo_runtime::tx()
            .velo_pay()
            .request_burn(amount_u128);

        let result = timeout(
            Duration::from_secs(BLOCKCHAIN_TIMEOUT_SECS),
            async {
                client
                    .tx()
                    .sign_and_submit_then_watch_default(&burn_call, &self.signer)
                    .await?
                    .wait_for_finalized_success()
                    .await
            }
        )
        .await
        .map_err(|_| anyhow!("Blockchain operation timed out after {} seconds", BLOCKCHAIN_TIMEOUT_SECS))??;

        let tx_hash = format!("0x{}", hex::encode(result.extrinsic_hash()));

        // Extract request_id from events
        let request_id = Self::extract_burn_request_id(&result)?;

        log::info!("Burn request submitted - tx_hash: {}, request_id: {}", tx_hash, request_id);
        log::debug!("Burn request amount: {}", amount);

        Ok((tx_hash, request_id))
    }

    /// Approve a burn request on the blockchain
    pub async fn approve_burn(
        &self,
        client: &VelocityClient,
        request_id: u64,
    ) -> Result<String> {
        let approve_call = velo_runtime::tx()
            .velo_pay()
            .approve_burn(request_id);

        let result = timeout(
            Duration::from_secs(BLOCKCHAIN_TIMEOUT_SECS),
            async {
                client
                    .tx()
                    .sign_and_submit_then_watch_default(&approve_call, &self.signer)
                    .await?
                    .wait_for_finalized_success()
                    .await
            }
        )
        .await
        .map_err(|_| anyhow!("Blockchain operation timed out after {} seconds", BLOCKCHAIN_TIMEOUT_SECS))??;

        let tx_hash = format!("0x{}", hex::encode(result.extrinsic_hash()));

        log::info!("Burn approved - request_id: {}", request_id);
        log::debug!("Burn approval tx_hash: {}", tx_hash);

        Ok(tx_hash)
    }

    /// Reject a burn request on the blockchain
    pub async fn reject_burn(
        &self,
        client: &VelocityClient,
        request_id: u64,
    ) -> Result<String> {
        let reject_call = velo_runtime::tx()
            .velo_pay()
            .reject_burn(request_id);

        let result = timeout(
            Duration::from_secs(BLOCKCHAIN_TIMEOUT_SECS),
            async {
                client
                    .tx()
                    .sign_and_submit_then_watch_default(&reject_call, &self.signer)
                    .await?
                    .wait_for_finalized_success()
                    .await
            }
        )
        .await
        .map_err(|_| anyhow!("Blockchain operation timed out after {} seconds", BLOCKCHAIN_TIMEOUT_SECS))??;

        let tx_hash = format!("0x{}", hex::encode(result.extrinsic_hash()));

        log::info!("Burn rejected - request_id: {}", request_id);
        log::debug!("Burn rejection tx_hash: {}", tx_hash);

        Ok(tx_hash)
    }

    /// Transfer tokens between accounts
    pub async fn transfer(
        &self,
        client: &VelocityClient,
        to: &str,
        amount: Decimal,
    ) -> Result<String> {
        let to_account = Self::parse_account_id(to)?;
        let amount_u128 = Self::decimal_to_balance(amount)?;

        // transfer takes (destination, amount)
        let transfer_call = velo_runtime::tx()
            .velo_pay()
            .transfer(to_account, amount_u128);

        let result = timeout(
            Duration::from_secs(BLOCKCHAIN_TIMEOUT_SECS),
            async {
                client
                    .tx()
                    .sign_and_submit_then_watch_default(&transfer_call, &self.signer)
                    .await?
                    .wait_for_finalized_success()
                    .await
            }
        )
        .await
        .map_err(|_| anyhow!("Blockchain operation timed out after {} seconds", BLOCKCHAIN_TIMEOUT_SECS))??;

        let tx_hash = format!("0x{}", hex::encode(result.extrinsic_hash()));
        let block_hash = result.block_hash();

        log::info!("Transfer submitted - tx_hash: {}, block: {:?}", tx_hash, block_hash);
        log::debug!("Transfer details - to: {}, amount: {}", to, amount);

        Ok(tx_hash)
    }

    /// Convert decimal amount to blockchain balance (assuming 12 decimals)
    fn decimal_to_balance(amount: Decimal) -> Result<u128> {
        // VeloPay uses 12 decimals (like DOT)
        let multiplier = Decimal::new(1_000_000_000_000, 0); // 10^12
        let balance = amount * multiplier;

        // Convert to string and parse as u128
        let balance_str = balance.to_string();
        let balance_parts: Vec<&str> = balance_str.split('.').collect();
        let integer_part = balance_parts.first()
            .ok_or_else(|| anyhow!("Invalid balance format"))?;

        integer_part.parse::<u128>()
            .map_err(|e| anyhow!("Failed to convert balance to u128: {}", e))
    }

    /// Parse account ID from SS58 address string
    fn parse_account_id(address: &str) -> Result<subxt::utils::AccountId32> {
        // Use sp_core's Ss58Codec trait to decode the address
        use sp_core::crypto::AccountId32 as SpAccountId32;

        let sp_account = SpAccountId32::from_ss58check(address)
            .map_err(|e| anyhow!("Invalid SS58 address '{}': {:?}", address, e))?;

        // Convert sp_core::AccountId32 to subxt::utils::AccountId32
        let bytes: [u8; 32] = sp_account.into();
        Ok(subxt::utils::AccountId32::from(bytes))
    }

    /// Extract mint request ID from blockchain events
    fn extract_mint_request_id(result: &subxt::blocks::ExtrinsicEvents<subxt::PolkadotConfig>) -> Result<u64> {
        // Find the MintRequested event
        let mint_requested_event = velo_runtime::velo_pay::events::MintRequested;

        for event in result.iter() {
            let event = event.map_err(|e| anyhow!("Failed to decode event: {:?}", e))?;

            if let Some(mint_event) = event.as_event::<velo_runtime::velo_pay::events::MintRequested>()? {
                return Ok(mint_event.request_id);
            }
        }

        Err(anyhow!("MintRequested event not found in transaction"))
    }

    /// Extract burn request ID from blockchain events
    fn extract_burn_request_id(result: &subxt::blocks::ExtrinsicEvents<subxt::PolkadotConfig>) -> Result<u64> {
        // Find the BurnReserved event (burn requests emit BurnReserved, not BurnRequested)
        for event in result.iter() {
            let event = event.map_err(|e| anyhow!("Failed to decode event: {:?}", e))?;

            if let Some(burn_event) = event.as_event::<velo_runtime::velo_pay::events::BurnReserved>()? {
                return Ok(burn_event.request_id);
            }
        }

        Err(anyhow!("BurnReserved event not found in transaction"))
    }
}
